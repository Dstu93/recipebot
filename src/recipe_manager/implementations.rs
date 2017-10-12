
use std::io::Error;

use recipe_manager::services::*;
use recipe_manager::objects::Recipe;
use recipe_manager::objects::Ingredient;
use recipe_manager::objects::Unit;

#[derive(Debug)]
pub struct FakeRecipeDBServiceImpl{
    recipes : Vec<Recipe>,
}

impl FakeRecipeDBServiceImpl{
    ///Creates Fake Service
    pub fn new() -> Result<FakeRecipeDBServiceImpl, Error>{
        let mut service = FakeRecipeDBServiceImpl{
            recipes : Vec::new(),
        };

        let mut schockokuchen = Recipe::new(1,String::from("Schockokuchen"), 
                String::from("Alles in eine Schüssel und schütteln"));
        let mut kaesekuchen = Recipe::new(2,String::from("Käsekuchen"), 
                String::from("Alles in eine Schüssel und schütteln"));
        let mut erdbeerkuchen = Recipe::new(3,String::from("Erdbeerkuchen"), 
                String::from("Alles in eine Schüssel und schütteln"));
        
        schockokuchen.add(Ingredient::new(String::from("Schokolade"),Unit::Kilogramm, 5.0));
        schockokuchen.add(Ingredient::new(String::from("Butter"),Unit::Kilogramm, 5.0));
        schockokuchen.add(Ingredient::new(String::from("Eier"),Unit::Quantity, 5.0));
        schockokuchen.add(Ingredient::new(String::from("Salz"),Unit::TeaSpoon, 3.5));

        kaesekuchen.add(Ingredient::new(String::from("Käse"),Unit::Kilogramm, 5.0));
        kaesekuchen.add(Ingredient::new(String::from("Butter"),Unit::Kilogramm, 5.0));
        kaesekuchen.add(Ingredient::new(String::from("Eier"),Unit::Quantity, 5.0));
        kaesekuchen.add(Ingredient::new(String::from("Salz"),Unit::TeaSpoon, 6.5));

        erdbeerkuchen.add(Ingredient::new(String::from("Erdbeeren"),Unit::Kilogramm, 5.0));
        erdbeerkuchen.add(Ingredient::new(String::from("Butter"),Unit::Kilogramm, 5.0));
        erdbeerkuchen.add(Ingredient::new(String::from("Eier"),Unit::Quantity, 5.0));
        erdbeerkuchen.add(Ingredient::new(String::from("Salz"),Unit::TeaSpoon, 3.5));

        
        service.add(schockokuchen).unwrap();
        service.add(kaesekuchen).unwrap();
        service.add(erdbeerkuchen).unwrap();
        Ok(service)
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