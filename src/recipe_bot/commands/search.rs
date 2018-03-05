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
use recipe_bot::commands::command::RecipeCommand;
use recipe_bot::commands::error::CmdError;

pub struct SearchCmd{
    dao: RwLock<Box<RecipeDAO + Send + Sync>>,
}

impl SearchCmd{
    pub fn new(dao: RwLock<Box<RecipeDAO + Send + Sync>>) -> SearchCmd{
        SearchCmd{dao}
    }
    
    pub fn exec_cmd(&self, args: Option<Vec<&str>>) -> Result<String,CmdError>{

        if args.is_none(){
            return Err(CmdError::NoArguments);
        }

        let lock_result = self.dao.read();
        if lock_result.is_err(){
            //FIXME log the error.
            return Err(CmdError::UnknownError);
        }

        let names = args.unwrap();
        if names.len() == 0 {
            return Err(CmdError::NoArguments);
        }

        let guard = lock_result.unwrap();
        let mut recipes: Vec<Recipe> = Vec::new();
        for name in names {
            let result = guard.find_by_name(&name.to_string());
            if result.is_ok(){recipes.extend(result.unwrap().into_iter())}
            else {
                //FIXME log Error
                println!("Error: searched for {}, found error: {:#?}",name,result.unwrap_err());
            }
        }

        if recipes.len() == 0 {
            return Err(CmdError::NotFound);
        }

        let mut answer = String::new();
        for recipe in recipes {
            let entry = format!("Id:{} Name: {}",recipe.id(),recipe.name());
            answer.push_str(&*entry);
            answer.push_str("\n")
        }
        Ok(answer)
    }

    /// translates the CmdError for this Command
    pub fn translate_error(e: &CmdError) -> &str{
        match e {
            &CmdError::NotFound => "Das Rezept wurde nicht gefunden",
            &CmdError::NoArguments => "Bitte geben sie die Namen der gesuchten Rezepte ",
            &CmdError::UnknownError => "Ein unbekannter Fehler ist aufgetreten",
            &CmdError::DatabaseAccessError => "Fehler beim Datenbank zugriff",
            &CmdError::InvalidInput => "Ihre eingabe ist ungültig.",
        }
    }

}

impl Command for SearchCmd{
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>){
        let result = self.exec_cmd(args);
        match result {
            Ok(answer) => {reply(bot,&update,&*answer);},
            Err(e) => {
                let error_descr = SearchCmd::translate_error(&e);
                reply(bot,&update,error_descr);
            },
        };
    }
}

impl RecipeCommand for SearchCmd{

    fn keyword(&self) -> &str{
        "search"
    }

    fn description(&self) -> &str{
        "Sucht ein Rezept anhand des Namens. Es werden alle Rezepte mit gleichen oder ähnlichen Namen angezeigt."
    }

    fn execute_cmd(&self, args: Option<Vec<&str>>) -> Result<String,CmdError>{
        self.exec_cmd(args)
    }

    fn execute_cmd_mut(&mut self, args: Option<Vec<&str>>) -> Result<String,CmdError>{
        self.execute_cmd(args)
    }

}

//util methods
//--------------------------------------------------

/// util method for reply to the user
fn reply(bot: &Bot,update: &Update,txt: &str){
    bot.reply_to_message(update, txt);
}