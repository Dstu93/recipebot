
use std::sync::RwLock;

use recipe_bot::util;
use recipe_manager::services::RecipeDAO;
use recipe_bot::commands::error::CmdError;
use recipe_bot::commands::command::RecipeCommand;

use teleborg::{Bot,Command};

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

        //TODO parse arguments

        let lock = self.dao.read();
        if lock.is_err(){
            return Err(CmdError::DatabaseAccessError);
        }

        let recipes = lock.unwrap().find_by_id()
    }

    fn execute_cmd_mut(&mut self, args: Option<Vec<&str>>) -> Result<String,CmdError>{
        self.execute_cmd(args)
    }

}

impl Command for DetailsCommand{
    fn execute(&mut self, bot: &bot::Bot, update: Update, args: Option<Vec<&str>>){
        let res = self.execute_cmd(args);
        let answer = match res {
            Ok(answer) => {&*answer},
            Err(e) => {util::translate_error(&e)}
        };
        bot.reply_to_message(&update,answer);
    }
}

