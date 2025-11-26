use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;

pub struct SaveCommand {

}

impl Command for SaveCommand {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
