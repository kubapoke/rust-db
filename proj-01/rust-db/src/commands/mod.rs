pub mod create;
pub mod insert;
pub mod delete;
pub mod select;
pub mod save;
pub mod read;

pub trait Command {}

pub enum AnyCommand {}