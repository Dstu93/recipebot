
use recipe_manager::services::{RecipeDBService, CSVExporter, GroceryListService};
use recipe_manager::implementations::FakeRecipeDBServiceImpl;

///ServiceFactory. Object to manage Services
pub struct ServiceFactory{}
impl ServiceFactory{
    
    /// builds an RecipeDBService
    pub fn recipe_service() -> Option<Box<RecipeDBService>>{
        let service :FakeRecipeDBServiceImpl = FakeRecipeDBServiceImpl::new().unwrap();
        Some(Box::new(service))
    }

    pub fn csv_exporter(&self) -> Option<Box<CSVExporter>>{
        Option::None
    }

    pub fn grocery_list_service(&self) -> Option<Box<GroceryListService>>{
        Option::None
    }
}

