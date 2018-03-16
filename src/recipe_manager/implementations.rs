
extern crate postgres;

use self::postgres::Connection;
use self::postgres::TlsMode;
use self::postgres::params::{Builder, Host};

use std::io::Error;

use recipe_manager::services::*;
use recipe_manager::objects::Recipe;
use recipe_manager::objects::Ingredient;
use recipe_manager::configuration::config::DatabaseConfig;

use serde_json;

/// RecipeDAO Implementation. Use a PostgreSQL Database
pub struct RecipeDAOImpl{
    config: DatabaseConfig,
}

impl RecipeDAOImpl{

    pub fn new(config: &DatabaseConfig) -> RecipeDAOImpl{
        RecipeDAOImpl{
            config: config.clone(),
        }
    }

    fn connect(&self) -> Result<Connection, Error>{
        let mut builder = Builder::new();
        builder.port(self.config.port());
        builder.user(&self.config.user(),Some(&self.config.password().unwrap()));
        builder.database(&self.config.database());
        let params = builder.build(Host::Tcp(self.config.host()));

        let conn = Connection::connect(params, TlsMode::None)?;
        Ok(conn)
    }

    fn parse_from_json(json: String) -> Result<Vec<Ingredient>, Error>{
        let ingredients: Vec<Ingredient> = serde_json::from_str(&json)?;
        Ok(ingredients)
    }

    fn parse_to_json(ingredients: Vec<Ingredient>) -> Result<String,Error>{
        let json = serde_json::to_string(&ingredients)?;
        Ok(json)
    }
    
}

impl RecipeDAO for RecipeDAOImpl{
     
    fn reciptes(&self) -> Result<Vec<Recipe>, Error>{
        //connect to our database
        let conn = self.connect()?;
        let mut recipes: Vec<Recipe> = Vec::new();

        //query all recipes and map them on our Recipe struct
        for row in &conn.query("SELECT id, recipe, description, persons ,ingredients FROM recipes", &[])? {
            
            let id = row.get(0);
            println!("id: {}", id);
            let name = row.get(1);
            println!("name: {}", name);
            let description = row.get(2);
            println!("descr: {}", description);
            let persons = row.get(3);
            println!("persons: {}", persons);
            
            //ingredients parsen
            let ingredient_json: String = row.get(4);
            let ingredients = RecipeDAOImpl::parse_from_json(ingredient_json)?;

            let recipe = Recipe::with_ingredients(id, name, description, persons,ingredients);
            recipes.push(recipe);
        }
        //After all we finish the connection and close it. 
        conn.finish()?;

        //if successfull return a vec of all our recipes
        Ok(recipes)
    }

    fn add(&mut self,recipe: &Recipe) -> Result<(), Error>{
            //Connect to the Database
            let conn = self.connect()?;
            let ingredient_json = RecipeDAOImpl::parse_to_json(recipe.ingredients())?;
            // Insert the Recipe in the Database
            conn.execute("INSERT INTO recipes (recipe,description, persons, path, ingredients) 
                    VALUES ($1,$2,$3,$4,$5)",
                &[&recipe.name(), &recipe.descr(),&recipe.persons(), &recipe.path(), &ingredient_json])?;
            conn.finish()?;
            Ok(())
    }

    fn add_all(&mut self, recipes: &Vec<Recipe>) -> Result<(),Error>{
        Ok(())
    }

    fn update(&mut self, recipe: &Recipe) -> Result<(),Error>{
        Ok(())
    }

    fn update_all(&mut self, recipes: &Vec<Recipe>) -> Result<(),Error>{
        Ok(())
    }

    fn delete(&mut self,recipe: &Recipe) -> Result<(), Error>{
        let conn = self.connect()?;
        let id = recipe.id();
        //TODO log out the number of affected rows.
        conn.execute("DELETE FROM recipes WHERE id = $1", &[&id])?;
        Ok(())
    }

    fn find_by_name(&self, name: &String) -> Result<Vec<Recipe>, Error>{
        let conn = self.connect()?;
        let mut recipes = Vec::new();
        for row in &conn.query("SElECT id, recipe, description, persons ,ingredients FROM recipes WHERE recipe = $1", &[&name])?{
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let descr: String = row.get(2);
            let persons: i16 = row.get(3);
            let ingr: String = row.get(4);
            let ingredients = RecipeDAOImpl::parse_from_json(ingr)?;

            recipes.push(Recipe::with_ingredients(id,name,descr,persons,ingredients));
        }
        Ok(recipes)
    }
    
    fn find_by_id(&self, id: &u32) -> Result<Option<Recipe>, Error>{
        let conn = self.connect()?;
        for row in &conn.query("SElECT id, recipe, description, persons ,ingredients FROM recipes WHERE id = $1", &[&id])?{
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let descr: String = row.get(2);
            let persons: i16 = row.get(3);
            let ingr: String = row.get(4);
            let ingredients = RecipeDAOImpl::parse_from_json(ingr)?;

            return Ok(Some(Recipe::with_ingredients(id,name,descr,persons,ingredients)));
        }
        Ok(None)
    }
}