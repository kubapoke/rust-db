use std::collections::HashMap;
use pest::iterators::Pair;
use pest::Parser;
use crate::errors::Error;
use crate::commands::command::AnyCommand;
use crate::commands::create::CreateCommand;
use crate::commands::insert::InsertCommand;
use crate::database::{Database, DatabaseKey, FieldType, IntermediateValue, Value};

#[derive(pest_derive::Parser)]
#[grammar = "commands.pest"]
struct QueryParser;

fn expect_any_rule<'a>(pair: Option<Pair<'a, Rule>>, msg: &'static str) -> Result<Pair<'a, Rule>, Error> {
    let pair = pair.ok_or_else(|| Error::NoTokenError(msg.into()))?;
    Ok(pair)
}

fn expect_rule<'a>(pair: Option<Pair<'a, Rule>>, expected: Rule, msg: &'static str) -> Result<Pair<'a, Rule>, Error> {
    let pair = pair.ok_or_else(|| Error::NoTokenError(msg.into()))?;
    if pair.as_rule() != expected {
        return Err(Error::UnknownTokenError(msg.into()));
    }
    Ok(pair)
}

pub fn parse_command<'a, K: DatabaseKey>(input: &str, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let mut pairs = match QueryParser::parse(Rule::command, input.trim()) {
        Ok(pairs) => pairs,
        Err(e) => {
            return Err(Error::ParseError(
                format!("Failed to parse command: {}", e.to_string())
            ));
        }
    };

    let command_pair = expect_rule(pairs.next(), Rule::command, "Expected a command")?;
    let query = expect_any_rule(command_pair.into_inner().next(), "Empty command")?;

    match query.as_rule() {
        Rule::create_query => { parse_create_query(query, database) },
        Rule::select_query => { parse_select_query(query, database) },
        Rule::insert_query => { parse_insert_query(query, database) },
        Rule::delete_query => { parse_delete_query(query, database) },
        Rule::read_query => { parse_read_query(query, database) },
        Rule::save_query => { parse_save_query(query, database) },
        _ => Err(Error::UnknownTokenError(String::from("Unexpected token in command")))
    }
}

pub fn parse_ident(ident_pair: Pair<Rule>) -> Result<String, Error> {
    let pair = expect_rule(Some(ident_pair), Rule::ident, "Expected an identifier")?;
    Ok(pair.as_str().to_string())
}

pub fn parse_numeric(numeric_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let pair = expect_rule(Some(numeric_pair), Rule::numeric, "Expected an numeric")?;
    let float = pair.as_str().parse::<f64>()
        .map_err(|e| Error::ParseError(format!("Failed to parse numeric: {}", e)))?;
    Ok(IntermediateValue::Numeric(float))
}

pub fn parse_string(string_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let pair = expect_rule(Some(string_pair), Rule::string, "Expected a string")?;
    let string = pair.as_str().to_string();
    Ok(IntermediateValue::String(string))
}

pub fn parse_bool(bool_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let pair = expect_rule(Some(bool_pair), Rule::bool, "Expected a boolean")?;
    let boolean = expect_any_rule(pair.into_inner().next(), "Expected a boolean value")?;
    match boolean.as_rule() {
        Rule::true_value => Ok(IntermediateValue::Bool(true)),
        Rule::false_value => Ok(IntermediateValue::Bool(false)),
        _ => Err(Error::UnknownTokenError(String::from("Unexpected token in bool")))
    }
}

pub fn parse_decl_type(decl_type_pair: Pair<Rule>) -> Result<FieldType, Error> {
    let type_pair = expect_any_rule(decl_type_pair.into_inner().next(), "Expected type declaration")?;

    match type_pair.as_rule() {
        Rule::int_type => Ok(FieldType::Int),
        Rule::float_type => Ok(FieldType::Float),
        Rule::string_type => Ok(FieldType::String),
        Rule::bool_type => Ok(FieldType::Bool),
        _ => Err(Error::UnknownTokenError(String::from("Unknown or invalid type")))
    }
}

pub fn parse_any_type_def(decl_type_pair: Pair<Rule>) -> Result<IntermediateValue, Error> {
    let type_pair = expect_any_rule(decl_type_pair.into_inner().next(), "Expected type declaration")?;

    match type_pair.as_rule() {
        Rule::numeric => parse_numeric(type_pair),
        Rule::string => parse_string(type_pair),
        Rule::bool => parse_bool(type_pair),
        _ => Err(Error::UnknownTokenError(String::from("Unknown or invalid type")))
    }
}

pub fn parse_create_query<'a, K: DatabaseKey>(create_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let items: Vec<_> = create_query_pair.into_inner().collect();

    let name_pair = expect_rule(items.get(1).cloned(), Rule::ident, "Missing or invalid name ident")?;
    let key_pair = expect_rule(items.get(3).cloned(), Rule::ident, "Missing or invalid key ident")?;
    let fields_pair = expect_rule(items.get(5).cloned(), Rule::decl_list, "Missing or invalid fields list")?;

    let name = parse_ident(name_pair)?;
    let key = parse_ident(key_pair)?;
    let fields = parse_decl_list(fields_pair)?;

    Ok(AnyCommand::Create(CreateCommand::new(database, name, key, fields)))
}

pub fn parse_decl_list(decl_list_pair: Pair<Rule>) -> Result<Vec<(String, FieldType)>, Error> {
    let mut fields = Vec::new();

    for decl_pair in decl_list_pair.into_inner() {
        let (key, field_type) = parse_decl(decl_pair)?;
        fields.push((key, field_type));
    }

    Ok(fields)
}

pub fn parse_decl(decl_pair: Pair<Rule>) -> Result<(String, FieldType), Error> {
    let mut decl = decl_pair.into_inner();

    let key_pair = expect_rule(decl.next(), Rule::ident, "Missing field name")?;
    let type_pair = expect_rule(decl.next(), Rule::decl_type, "Missing field type")?;

    let key = parse_ident(key_pair)?;
    let field_type = parse_decl_type(type_pair)?;

    Ok((key, field_type))
}

pub fn parse_select_query<'a, K: DatabaseKey>(select_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    let items: Vec<_> = select_query_pair.into_inner().collect();

    let assign_list_pair = expect_rule(items.get(1).cloned(), Rule::assign_list, "Missing or invalid assignment list")?;
    let table_ident_pair = expect_rule(items.get(3).cloned(), Rule::ident, "Missing or invalid table identifier")?;

    let assign_list = parse_assign_list(assign_list_pair)?;
    let table_id = parse_ident(table_ident_pair)?;

    let table = database.get_table(&table_id)?;

    Ok(AnyCommand::Insert(InsertCommand::new(table, assign_list)))
}

pub fn parse_assign_list(assign_list_pair: Pair<Rule>) -> Result<Vec<(String, IntermediateValue)>, Error> {
    let mut assignments = Vec::new();

    for assign_pair in assign_list_pair.into_inner() {
        let (key, value) = parse_assign(assign_pair)?;
        assignments.push((key, value));
    }

    Ok(assignments)
}

pub fn parse_assign(assign_pair: Pair<Rule>) -> Result<(String, IntermediateValue), Error> {
    let mut assign = assign_pair.into_inner();

    let key_pair = expect_rule(assign.next(), Rule::ident, "Mising or invalid field name")?;
    let type_def_pair = expect_rule(assign.next(), Rule::any_type_def, "Missing field type")?;

    let key = parse_ident(key_pair)?;
    let value = parse_any_type_def(type_def_pair)?;

    Ok((key, value))
}

pub fn parse_insert_query<'a, K: DatabaseKey>(insert_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    todo!()
}

pub fn parse_delete_query<'a, K: DatabaseKey>(delete_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    todo!()
}

pub fn parse_read_query<'a, K: DatabaseKey>(read_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    todo!()
}

pub fn parse_save_query<'a, K: DatabaseKey>(save_query_pair: Pair<Rule>, database: &'a mut Database<K>) -> Result<AnyCommand<'a, K>, Error> {
    todo!()
}
