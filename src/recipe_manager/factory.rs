
use recipe_manager::services::{RecipeDAO, CSVExporter, GroceryListService};

///ServiceFactory. Object to manage Services
pub struct ServiceFactory{}
impl ServiceFactory{
    
    /// builds an RecipeDBService
    pub fn recipe_service() -> Option<Box<RecipeDAO>>{
        Option::None
    }

    pub fn csv_exporter(&self) -> Option<Box<CSVExporter>>{
        Option::None
    }

    pub fn grocery_list_service(&self) -> Option<Box<GroceryListService>>{
        Option::None
    }
}

