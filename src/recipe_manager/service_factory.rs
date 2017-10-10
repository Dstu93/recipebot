
use recipe_manager::services::{RecipeDBService, CSVExporter, GroceryListService};
use recipe_manager::implementations::FakeRecipeDBServiceImpl;
use std::io::Error;

///ServiceFactory. Object to manage Services
pub struct ServiceFactory{
    recipe_service : Option<Box<RecipeDBService>>,
    csv_exporter : Option<Box<CSVExporter>>,
    grocery_list_service : Option<Box<GroceryListService>>,
}

impl ServiceFactory{
    ///Creates a new Factory
    pub fn new() -> Result<ServiceFactory, Error> {
        Ok(ServiceFactory{
            recipe_service : Option::Some(Box::new(FakeRecipeDBServiceImpl::new().unwrap())),
            csv_exporter : Option::None,
            grocery_list_service : Option::None,
        })
    }

    //Methoden impl

    pub fn recipe_service(&self) -> &Box<RecipeDBService>{
        match &self.recipe_service{
           &Some(ref service_box) => {
                service_box
            }
            &None => {
                panic!("RecipeDBService is not initialyzed");
            }
        }
    }

    pub fn csv_exporter(&self) -> &Box<CSVExporter>{
        match &self.csv_exporter{
           &Some(ref service_box) => {
                service_box
            }
            &None => {
                panic!("CSVExporter is not initialyzed");
            }
        }
    }

    pub fn grocery_list_service(&self) -> &Box<GroceryListService>{
        match &self.grocery_list_service{
           &Some(ref service_box) => {
                service_box
            }
            &None => {
                panic!("GroceryListService is not initialyzed");
            }
        }
    }
}

