
use recipe_manager::objects::{Recipe, Ingredient};

use std::io::Error;

///Service for Collect Ingredients to make a GroceryList
pub trait GroceryListService{
    fn collect_ingredients(&self, recipes : Vec<Recipe>) -> Vec<Ingredient>;
}

///Object for Database Access to store and access Recipes
pub trait RecipeDBService {
    fn reciptes(&mut self) -> Result<Vec<Recipe>, Error>;
    fn find(&mut self, name : String) -> Result<Vec<Recipe>, Error>; 
    fn add(&mut self,recipe : Recipe) -> Result<(), Error>;
    fn delete(&mut self,recipe : Recipe) -> Result<(), Error>;
}

pub trait CSVExporter{
    fn export(&mut self, ingredients : Vec<Ingredient>, filename : String) -> Result<(), Error>;
}