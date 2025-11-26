use std::collections::HashMap;
use pest::iterators::Pair;
use pest::Parser;
use crate::errors::Error;
use crate::commands::command::AnyCommand;
use crate::commands::create::CreateCommand;
use crate::database::{AnyDatabase, FieldType, Value};

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

pub fn parse_command<'a>(input: &str, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
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

pub fn parse_create_query<'a>(create_query_pair: Pair<Rule>, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    let items: Vec<_> = create_query_pair.into_inner().collect();

    let name_pair = expect_rule(items.get(1).cloned(), Rule::ident, "Missing or invalid name ident")?;
    let key_pair = expect_rule(items.get(3).cloned(), Rule::ident, "Missing or invalid key ident")?;
    let fields_pair = expect_rule(items.get(5).cloned(), Rule::decl_list, "Missing or invalid fields list")?;

    let name = parse_ident(name_pair)?;
    let key = parse_ident(key_pair)?;
    let fields = parse_decl_list(fields_pair)?;

    let key_field_type = fields.get(&key)
        .ok_or_else(|| Error::NotSpecifiedError("Field type of key was not specified".into()))?;

    if *key_field_type != database.key_type() {
        return Err(Error::TypeError("Invalid key field type".into()));
    }

    Ok(AnyCommand::Create(CreateCommand::new(database, name, key, fields)))
}

pub fn parse_decl_list(decl_list_pair: Pair<Rule>) -> Result<HashMap<String, FieldType>, Error> {
    let mut fields = HashMap::new();

    for decl_pair in decl_list_pair.into_inner() {
        let (key, field_type) = parse_decl(decl_pair)?;

        if fields.contains_key(&key) {
            return Err(Error::AlreadyExistsError(
                format!("Field '{}' is declared more than once", key)
            ));
        }

        fields.insert(key, field_type);
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

pub fn parse_select_query<'a>(select_query_pair: Pair<Rule>, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    todo!()
}

pub fn parse_insert_query<'a>(insert_query_pair: Pair<Rule>, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    todo!()
}

pub fn parse_delete_query<'a>(delete_query_pair: Pair<Rule>, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    todo!()
}

pub fn parse_read_query<'a>(read_query_pair: Pair<Rule>, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    todo!()
}

pub fn parse_save_query<'a>(save_query_pair: Pair<Rule>, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    todo!()
}
