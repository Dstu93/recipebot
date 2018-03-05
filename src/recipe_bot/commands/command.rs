
use recipe_bot::commands::error::CmdError;

/// Commands for the RecipeBot.
pub trait RecipeCommand{

    /// returns the keyword which should be used for calling this command
    fn keyword(&self) -> &str;

    /// returns an description of this command for the user
    fn description(&self) -> &str;

    /// executes this command non mutable, returns
    /// an result. In case of success it returns the answer as string, if an error
    /// occurs the funcion will return an CommandError
    fn execute_cmd(&self, args: Option<Vec<&str>>) -> Result<String,CmdError>;

    /// executes this command as mutable, returns
    /// an result. In case of success it returns the answer as string, if an error
    /// occurs the funcion will return an CommandError
    fn execute_cmd_mut(&mut self, args: Option<Vec<&str>>) -> Result<String,CmdError>;
}