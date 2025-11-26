use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;

pub struct SelectCommand {

}

impl Command for SelectCommand {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
