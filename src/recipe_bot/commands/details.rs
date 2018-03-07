
use std::sync::RwLock;

use recipe_manager::services::RecipeDAO;
use recipe_bot::commands::error::CmdError;
use recipe_bot::commands::command::RecipeCommand;

use teleborg::Command;

/// Details command for the recipe-bot.
/// takes one or more ids of recipes and returns the recipe details as string.
pub struct DetailsCommand{
    dao: RwLock<Box<RecipeDAO>>,
}

impl DetailsCommand{

    /// creates a new DetailsCommand-Object
    pub fn new(dao: RwLock<Box<RecipeDAO>>) -> DetailsCommand{
        DetailsCommand{dao}
    }
}

impl RecipeCommand for DetailsCommand{

    fn keyword(&self) -> &str{
        "details"
    }

    fn description(&self) -> &str{
        "Nimmt die ID von einem oder mehreren Rezepten entgegen und gibt die Details der Rezepte zurück."
    }

    fn execute_cmd(&self, args: Option<Vec<&str>>) -> Result<String,CmdError>{
        //TODO writing code
    }

    fn execute_cmd_mut(&mut self, args: Option<Vec<&str>>) -> Result<String,CmdError>{
        self.execute_cmd(args)
    }

}

impl Command for DetailsCommand{
    fn execute(&mut self, bot: &bot::Bot, update: Update, args: Option<Vec<&str>>){
        //TODO reply in util Methode packen und Translation ebenfalls.
    }
}

