use crate::commands::AnyCommand;
use crate::errors::Error;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "queries.pest"]
struct QueryParser;

pub fn parse_command(input: &str) -> Result<AnyCommand, Error> {
    let pairs = match QueryParser::parse(Rule::command, input.trim()) {
        Ok(pairs) => pairs,
        Err(e) => { return  Err(Error::ParseError(input.to_string())) }
    };

    todo!()
}