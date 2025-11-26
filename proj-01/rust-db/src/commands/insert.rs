use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;

pub struct InsertCommand {

}

impl Command for InsertCommand {
    fn execute() -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
