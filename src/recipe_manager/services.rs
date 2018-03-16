
use recipe_manager::objects::{Recipe, Ingredient};

use std::io::Error;
use std::thread::JoinHandle;

///Service for Collect Ingredients to make a GroceryList
pub trait GroceryListService{
    /// Collects all Ingredients of all given Recipes and merge them to one Grocery_list
    fn grocery_list(&self, recipes: Vec<Recipe>) -> Vec<Ingredient>;
    /// Exports the Ingredient List as CSV
    fn export_to_csv(&mut self, ingredients: &Vec<Ingredient>, filename: &String) -> Result<(), Error>;
}

///Object for Database Access to store and access Recipes
pub trait RecipeDAO {
    /// Get All Recipes in the Database
    fn reciptes(&self) -> Result<Vec<Recipe>, Error>;
    /// Gets all Recipes that contains the given String in the name
    fn find_by_name(&self, name: &String) -> Result<Vec<Recipe>, Error>;
    /// returns the Recipe with the given Id
    fn find_by_id(&self, id: &u32) -> Result<Option<Recipe>, Error>;
    /// Adds a Recipe to the Database, the id will be ignored and generated by the database/service 
    fn add(&mut self,recipe: &Recipe) -> Result<(), Error>;
    /// Adds a collection of Recipes to the database, the id will be ignored and generate by the database/service
    fn add_all(&mut self, recipes: &Vec<Recipe>) -> Result<(),Error>;
    /// Updates the Recipe with the given id.
    fn update(&mut self, recipe: &Recipe) -> Result<(),Error>;
    /// Updates all Recipes with the given id.
    fn update_all(&mut self, recipes: &Vec<Recipe>) -> Result<(),Error>;
    /// Deletes the Recipe
    fn delete(&mut self,recipe: &Recipe) -> Result<(), Error>;
}

/// Service for importing Recipes into the Database
pub trait RecipeWebService {

    /// Starts the RecipeWebService. The RecipeWebService is an API 
    /// for other clients and use Http/Https 
    fn start(&self) -> Result<JoinHandle<()>,Error>;

    /// Stops and close the Web API
    fn stop(&self) -> Result<(),Error>;
}