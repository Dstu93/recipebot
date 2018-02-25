
use std::sync::RwLock;

use teleborg::{Dispatcher,Command};

use recipe_manager::services::RecipeDAO;
use recipe_manager::configuration::config::ApplicationConfig;

/// RecipeBot
pub struct RecipeBot{
    recipe_dao: RwLock<Box<RecipeDAO>>,
    api_key: String,
}

//TODO use Errors instead of Strings in Result
impl RecipeBot{

    /// creates a new instance of the recipe bot
    pub fn new(recipe_dao: Box<RecipeDAO>, api_key: String) -> RecipeBot{
        let recipe_dao = RwLock::new(recipe_dao);
        RecipeBot{recipe_dao, api_key }
    }

    /// starts the bot
    pub fn start(&self) -> Result<(),String>{
        //TODO need too instance all recipe commands and register them

        Ok(())
    }

    /// stops the bot
    pub fn stop(&mut self) -> Result<(),String>{
        Ok(())
    }

}