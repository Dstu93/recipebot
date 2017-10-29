
use recipe_manager::services::{RecipeDAO, GroceryListService, RecipeWebService};

/// ServiceFactory. Object to manage Services
pub struct ServiceFactory{
    
}

impl ServiceFactory{
    
    /// builds an RecipeDBService
    pub fn recipe_service(&self) -> Option<Box<RecipeDAO>>{
        Option::None
    }

    pub fn grocery_list_service(&self) -> Option<Box<GroceryListService>>{
        Option::None
    }

    pub fn recipe_web_service(&self) -> Option<Box<RecipeWebService>>{
        Option::None
    }

}

