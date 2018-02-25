// SearchCmd -> Search recipes by name and print list of recipes with ids
// DetailsCmd -> list details
// PicturesCmd -> Sends picture of a given recipe
// TreeCmd -> Sends an Tree of Recipe Categories
// TraverseTreeCmd -> Traverse Nodes of the Recipe Tree, search given path
// CalcFormCmd -> calculates ingredients to the given form
// CalcQuantityCmd -> calculates ingredients by quantity
// GroceryListCmd -> Calculates ingredients from all given recipes

use std::sync::RwLock;
use teleborg::{Command,Bot};
use teleborg::objects::Update;
use recipe_manager::services::RecipeDAO;
use recipe_manager::objects::*;
use recipe_bot::commands::error::CmdError;

pub struct SearchCmd{
    dao: RwLock<Box<RecipeDAO + Send + Sync>>,
}

impl SearchCmd{
    pub fn new(dao: RwLock<Box<RecipeDAO + Send + Sync>>) -> SearchCmd{
        SearchCmd{dao}
    }

    //FIXME create Error Enum for the commands
    pub fn exec_cmd(&self, args: Option<Vec<&str>>) -> Result<String,CmdError>{

        if args.is_none(){
            return Err(CmdError::NoArguments);
        }

        let lock_result = self.dao.read();
        if lock_result.is_err(){
            //TODO log the error.
            return Err(CmdError::UnknownError);
        }

        let names = args.unwrap();
        if names.len() == 0 {
            return Err(CmdError::NoArguments);
        }

        let guard = lock_result.unwrap();
        let mut recipes = Vec::new();
        for name in names {
            let result = guard.find_by_name(&name.to_string());
            if result.is_ok(){recipes.extend(result.unwrap().into_iter())}
            else {
                //TODO log Error
                println!("Error: searched for {}, found error: {:#?}",name,result.unwrap_err());
            }
        }

        if recipes.len() == 0 {
            return Err(CmdError::NotFound);
        }

        //TODO create String List from all recipes
        let list = String::from("TODO");
        Ok(list)
    }
}

impl Command for SearchCmd{
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>){
        let result = self.exec_cmd(args);
        if result.is_ok(){
            reply(bot,&update,&*result.unwrap());
        } else{
            //TODO Match Error Enum and reply
        }
    }
}

//util methods
//--------------------------------------------------

/// util method for reply to the user
fn reply(bot: &Bot,update: &Update,txt: &str){
    bot.reply_to_message(update, txt);
}