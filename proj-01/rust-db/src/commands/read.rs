use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;

pub struct ReadCommand {

}

impl Command for ReadCommand {
    fn execute() -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
