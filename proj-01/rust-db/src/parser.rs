use pest::Parser;
use crate::errors::Error;
use crate::commands::command::AnyCommand;

#[derive(pest_derive::Parser)]
#[grammar = "commands.pest"]
struct QueryParser;

pub fn parse_command(input: &str) -> Result<AnyCommand, Error> {
    let mut pairs = match QueryParser::parse(Rule::command, input.trim()) {
        Ok(pairs) => pairs,
        Err(_) => { return  Err(Error::ParseError(input.to_string())) }
    };

    let command_pair = match pairs.next() {
        Some(p) => p,
        None => { return Err(Error::NoMatchError) }
    };

    match command_pair.as_rule()      {
        Rule::CREATE => {println!("create")},
        Rule::SELECT => {println!("select")},
        Rule::INSERT => {println!("insert")},
        Rule::DELETE => {println!("delete")},
        Rule::READ_FROM => {println!("read_from")},
        Rule::SAVE_AS => {println!("save_as")},
        _ => { return  Err(Error::ParseError(input.to_string())) }
    }

    todo!()
}
