
use std::io::Error;

use recipe_manager::configuration::config::*;
use recipe_manager::services::{RecipeDAO, GroceryListService, RecipeWebService};
use recipe_manager::implementations::*;
use web_api::api::RecipeWebServiceImpl;

/// ServiceFactory crates needed services for this application
pub struct ServiceFactory;


impl ServiceFactory{

    /// builds an RecipeDBService
    pub fn recipe_service(config: &DatabaseConfig) -> Result<Box<RecipeDAO + Send + Sync>,Error>{
        let service = RecipeDAOImpl::new(config);
        Ok(Box::new(service))
    }

    pub fn recipe_web_service(config: &ApplicationConfig) -> Result<Box<RecipeWebService>,Error>{
        Ok(Box::new(RecipeWebServiceImpl::new(config.clone())))
    }

}

