
use std::io::Error;

use recipe_manager::services::*;
use recipe_manager::objects::Recipe;

pub struct FakeRecipeDBServiceImpl{
    recipes : Vec<Recipe>,
}

impl FakeRecipeDBServiceImpl{
    ///Creates Fake Service
    pub fn new() -> Result<FakeRecipeDBServiceImpl, Error>{
        Ok(FakeRecipeDBServiceImpl{
            recipes : Vec::new(),
        })
    }

}

impl RecipeDBService for FakeRecipeDBServiceImpl{
    fn reciptes(&mut self) -> Result<Vec<Recipe>, Error>{
        Ok(self.recipes.clone())
    }

    fn find(&mut self, name : String) -> Result<Vec<Recipe>, Error>{
        Ok(self.recipes.clone())
    } 

    fn add(&mut self,recipe : Recipe) -> Result<(), Error>{
        self.recipes.push(recipe);
        Ok(())
    }
    fn delete(&mut self,recipe : Recipe) -> Result<(), Error>{
        Ok(())
    }
}