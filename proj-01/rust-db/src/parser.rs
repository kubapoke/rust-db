use pest::iterators::Pair;
use pest::Parser;
use crate::errors::Error;
use crate::commands::command::AnyCommand;
use crate::database::AnyDatabase;

#[derive(pest_derive::Parser)]
#[grammar = "commands.pest"]
struct QueryParser;

pub fn parse_command_list(input: &str, database: &mut AnyDatabase) -> Result<Vec<AnyCommand>, Error> {
    let mut pairs = match QueryParser::parse(Rule::command_list, input.trim()) {
        Ok(pairs) => pairs,
        Err(_) => { return  Err(Error::ParseError(input.to_string())) }
    };

    let command_list_pair = match pairs.next() {
        None => { return Err(Error::NoMatchError) },
        Some(p) => p
    };

    let mut commands = Vec::new();
    for command in command_list_pair.into_inner() {
        if command.as_rule() == Rule::command {
            commands.push(parse_command(command, database)?);
        }
    }

    Ok(commands)
}

pub fn parse_command(command_pair: Pair<Rule>, database: &mut AnyDatabase) -> Result<AnyCommand, Error> {
    let query = match command_pair.into_inner().next() {
        None => { return Err(Error::NoMatchError) },
        Some(i) => i
    };

    match query.as_rule()      {
        Rule::create_query => { parse_create_query(query, database) },
        Rule::select_query => { parse_select_query(query, database) },
        Rule::insert_query => { parse_insert_query(query, database) },
        Rule::delete_query => { parse_delete_query(query, database) },
        Rule::read_query => { parse_read_query(query, database) },
        Rule::save_query => { parse_save_query(query, database) },
        _ => Err(Error::ParseError(String::from(query.as_str())))
    }
}

pub fn parse_create_query(create_query_pair: Pair<Rule>, database: &mut AnyDatabase) -> Result<AnyCommand, Error> {
    todo!()
}

pub fn parse_select_query(select_query_pair: Pair<Rule>, database: &mut AnyDatabase) -> Result<AnyCommand, Error> {
    todo!()
}

pub fn parse_insert_query(insert_query_pair: Pair<Rule>, database: &mut AnyDatabase) -> Result<AnyCommand, Error> {
    todo!()
}

pub fn parse_delete_query(delete_query_pair: Pair<Rule>, database: &mut AnyDatabase) -> Result<AnyCommand, Error> {
    todo!()
}

pub fn parse_read_query(read_query_pair: Pair<Rule>, database: &mut AnyDatabase) -> Result<AnyCommand, Error> {
    todo!()
}

pub fn parse_save_query(save_query_pair: Pair<Rule>, database: &mut AnyDatabase) -> Result<AnyCommand, Error> {
    todo!()
}
