
/// Enum for representing errors for a Bot-Command
#[derive(Debug,PartialEq,Eq,Copy,Hash,Clone)]
pub enum CmdError{
    NotFound,
    NoArguments,
    InvalidInput,
    UnknownError,
    DatabaseAccessError,
}

//TODO add fn for bot answers for this specific error of a command