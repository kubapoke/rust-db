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

pub enum AnyCommand<'a> {
    Create(CreateCommand<'a>),
    Delete(DeleteCommand),
    Insert(InsertCommand),
    Read(ReadCommand),
    Save(SaveCommand),
    Select(SelectCommand),
}

impl Command for AnyCommand<'_> {
    fn execute(&mut self) -> Result<ExecutionSuccessValue, Error> {
        match self { 
            AnyCommand::Create(c) => c.execute(),
            AnyCommand::Delete(c) => c.execute(),
            AnyCommand::Insert(c) => c.execute(),
            AnyCommand::Read(c) => c.execute(),
            AnyCommand::Save(c) => c.execute(),
            AnyCommand::Select(c) => c.execute(),
        }
    }
}

pub enum ExecutionSuccessValue {
    Success,
}
