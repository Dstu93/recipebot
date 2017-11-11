
use std::io::Error;

use recipe_manager::configuration::config::*;
use recipe_manager::services::{RecipeDAO, GroceryListService, RecipeWebService};
use recipe_manager::implementations::*;
use web_api::api::RecipeWebServiceImpl;

/// ServiceFactory. Object to manage Services
pub struct ServiceFactory{
    config: ApplicationConfig,
}


impl ServiceFactory{

    pub fn new(config: ApplicationConfig) -> ServiceFactory{
        ServiceFactory{
            config: config,
        }
    }
    
    /// builds an RecipeDBService
    pub fn recipe_service(&self) -> Result<Box<RecipeDAO>,Error>{
        let service = RecipeDAOImpl::new(&self.config.database_config());
        Ok(Box::new(service))
    }

    pub fn grocery_list_service(&self) -> Result<Box<GroceryListService>,Error>{
        unimplemented!("currently not supported");
    }

    pub fn recipe_web_service(&self) -> Result<Box<RecipeWebService>,Error>{
        Ok(Box::new(RecipeWebServiceImpl::new(self.config.clone())))
    }

}

