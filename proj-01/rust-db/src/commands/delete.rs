use crate::commands::command::{Command, ExecutionSuccessValue};
use crate::errors;

pub struct DeleteCommand {

}

impl Command for DeleteCommand {
    fn execute() -> Result<ExecutionSuccessValue, errors::Error> {
        todo!()
    }
}
