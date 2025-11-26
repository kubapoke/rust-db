use crate::commands::create::CreateCommand;
use crate::commands::delete::DeleteCommand;
use crate::commands::insert::InsertCommand;
use crate::commands::read::ReadCommand;
use crate::commands::save::SaveCommand;
use crate::commands::select::SelectCommand;
use crate::errors::Error;

pub trait Command {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, Error>;
}

pub enum AnyCommand {
    Create(CreateCommand),
    Delete(DeleteCommand),
    Insert(InsertCommand),
    Read(ReadCommand),
    Save(SaveCommand),
    Select(SelectCommand),
}

pub enum ExecutionSuccessValue {
    Success,
}
