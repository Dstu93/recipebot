
use std::sync::RwLock;

use recipe_bot::util;
use recipe_manager::services::RecipeDAO;
use recipe_bot::commands::error::CmdError;
use recipe_bot::commands::command::RecipeCommand;

use teleborg::{Bot,Command};
use teleborg::objects::Update;

/// Details command for the recipe-bot.
/// takes one or more ids of recipes and returns the recipe details as string.
pub struct DetailsCommand{
    dao: RwLock<Box<RecipeDAO + Send + Sync>>,
}

impl DetailsCommand{

    /// creates a new DetailsCommand-Object
    pub fn new(dao: RwLock<Box<RecipeDAO + Send + Sync>>) -> DetailsCommand{
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

        if args.is_none(){
            return Err(CmdError::NoArguments);
        }

        let mut ids: Vec<u32> = Vec::new();
        let args = args.unwrap();
        for arg in args {

        }


        let lock = self.dao.read();
        if lock.is_err(){
            return Err(CmdError::DatabaseAccessError);
        }
        let dao = lock.unwrap();
        let mut recipes = Vec::with_capacity(ids.len());
        for id in &ids {
            let res = dao.find_by_id(id);
            if res.is_ok(){ recipes.push(res.unwrap());}
            //TODO Error Message when recipe with id was not found
        }

        let mut answer = String::with_capacity(50 * ids.len()); //maybe with precalculated capacity?
        //TODO adding Recipes to answer

        Ok(answer)
    }

    fn execute_cmd_mut(&mut self, args: Option<Vec<&str>>) -> Result<String,CmdError>{
        self.execute_cmd(args)
    }

}

impl Command for DetailsCommand{
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>){
        let res = self.execute_cmd(args);
        match res {
            Ok(answer) => {
                bot.reply_to_message(&update,&*answer);
            },
            Err(e) => {
                let answer = util::translate_error(&e);
                bot.reply_to_message(&update,answer);
            }
        }
    }
}

