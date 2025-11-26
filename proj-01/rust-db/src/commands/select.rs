use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;

pub struct SelectCommand {

}

impl Command for SelectCommand {
    fn execute() -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
