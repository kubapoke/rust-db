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

pub fn parse_command<'a>(input: &str, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    let mut pairs = match QueryParser::parse(Rule::command, input.trim()) {
        Ok(pairs) => pairs,
        Err(e) => {
            return Err(Error::ParseError(
                format!("Failed to parse command: {}", e.to_string())
            ));
        }
    };

    let command_pair = match pairs.next() {
        None => { return Err(Error::NoTokenError(String::from("Expected a command, but found nothing"))); },
        Some(p) => p
    };

    let query = match command_pair.into_inner().next() {
        None => { return Err(Error::NoTokenError(String::from("Command found, but contains no query"))); },
        Some(i) => i
    };

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
    match ident_pair.as_rule() {
        Rule::ident => Ok(ident_pair.as_str().to_string()),
        _ => Err(Error::UnknownTokenError(String::from("Expected an identifier")))
    }
}

pub fn parse_decl_type(decl_type_pair: Pair<Rule>) -> Result<FieldType, Error> {
    let type_pair = match decl_type_pair.into_inner().next() {
        None => { return Err(Error::NoTokenError(String::from("Expected a type declaration"))); },
        Some(t) => t
    };

    match type_pair.as_rule() {
        Rule::int_type => Ok(FieldType::Int),
        Rule::float_type => Ok(FieldType::Float),
        Rule::string_type => Ok(FieldType::String),
        Rule::bool_type => Ok(FieldType::Bool),
        _ => Err(Error::UnknownTokenError(String::from("Unknown or invalid type")))
    }
}

pub fn parse_create_query<'a>(create_query_pair: Pair<Rule>, database: &'a mut AnyDatabase) -> Result<AnyCommand<'a>, Error> {
    let query = create_query_pair.into_inner();

    let mut ident_count = 0;
    let mut name = String::new();
    let mut key = String::new();
    let mut fields = HashMap::new();

    for pair in query {
        match pair.as_rule() {
            Rule::ident => {
                if ident_count == 0 { name = parse_ident(pair)?; }
                else if ident_count == 1 { key = parse_ident(pair)?; }
                else {
                    return Err(Error::UnknownTokenError(
                        String::from("Too many identifiers in CREATE statement")
                    ));
                }
                ident_count += 1;
            }
            Rule::decl_list => { fields = parse_decl_list(pair)?; },
            Rule::CREATE | Rule::KEY | Rule::FIELDS => {}
            _ => {
                return Err(Error::UnknownTokenError(
                    String::from("Unexpected token in CREATE query")
                ));
            }
        }
    }

    if ident_count != 2 {
        return Err(Error::MissingTokenError(
            String::from("CREATE query must contain exactly two identifiers: table name and key name")
        ));
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
    let key: String;
    let field_type: FieldType;

    let mut decl = decl_pair.into_inner();

    key = match decl.next() {
        None => { return Err(Error::NoTokenError(String::from("Field declaration missing name"))); },
        Some(i) => parse_ident(i)?
    };

    field_type = match decl.next() {
        None => { return Err(Error::NoTokenError(String::from("Field declaration missing type"))); },
        Some(t) => parse_decl_type(t)?
    };

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
