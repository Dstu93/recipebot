
use recipe_manager::traits::RecipeFmt;

///Object to represent a recipe
#[derive(Debug,Clone,PartialEq)]
pub struct Recipe{
    id : u32,
    name : String,
    descr : String,
    ingredients : Vec<Ingredient>,
}

#[allow(dead_code)]
impl Recipe{
    /// Creates a new Recipe. Ingredients must add after creation.
    pub fn new(id : u32, name : String, descr : String) -> Recipe{
        Recipe{
            id : id,
            name : name,
            descr : descr,
            ingredients : Vec::new(),
        }
    }

    /// Returns the Id of the Recipe
    pub fn id(&self) -> u32{
        self.id
    }

    /// Returns the name of the Recipe
    pub fn name(&self) -> String{
        self.name.clone()
    }

    /// Description of the Recipe
    pub fn descr(&self) -> String{
        self.descr.clone()
    }

    /// Returns a Vector of all Ingredients that the Recipe owns
    pub fn ingredients(&self) -> Vec<Ingredient>{
        self.ingredients.clone()
    }

    /// Adds an Ingredient to the Recipe
    pub fn add(&mut self, ingredient : Ingredient){
        &self.ingredients.push(ingredient);
    }
}

impl RecipeFmt for Recipe{
    fn display(&self) -> String{
        let mut display_value = String::new();
        display_value.push_str(&self.name);
        new_line(&mut display_value);
        display_value.push_str(&self.descr);
        new_line(&mut display_value);
        for ingredient in &self.ingredients{
            display_value.push_str(&ingredient.display());
            new_line(&mut display_value);
        }
        display_value
    }
}

//---------------------------------------------------------------------------

/// represents an Ingredient of a Recipe
#[derive(Debug,Clone,PartialEq)]
pub struct Ingredient{
    name : String,
    unit : Unit,
    quantity : f32,
}

#[allow(dead_code)]
impl Ingredient{
    /// Creates a new Ingredient.
    pub fn new(name : String, unit : Unit, quantity : f32) -> Ingredient{
        Ingredient{
            name : name,
            unit : unit,
            quantity : quantity,
        }
    }

    /// Gets the name of the Ingredient.
    pub fn name(&self) -> &String{
        &self.name
    }

    /// returns the messerument Unit of the Ingredient.
    pub fn unit(&self) -> Unit{
        self.unit
    }

    /// The Quantity of the Ingredient and of the Unit
    pub fn quantity(&self) -> f32{
        self.quantity
    }
}

impl RecipeFmt for Ingredient{
    fn display(&self) -> String{
       let fmt = format!("{} {}{}", &self.name,self.quantity, self.unit.display());
       fmt
    }
}

//---------------------------------------------------------------------------

/// Enum of Unit Types of Ingredients
#[derive(Debug,Clone, Copy, Hash, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Unit{
    Gramm,
    Kilogramm,
    Milliliter,
    Liter,
    Quantity,
    TeaSpoon,
    Spoon,
}

impl RecipeFmt for Unit{

    /// Gibt das Einheitskuerzel zurueck.
    fn display(&self) -> String{
        match self{
            &Unit::Gramm => {String::from(" g")},
            &Unit::Kilogramm => {String::from(" kg")},
            &Unit::Milliliter => {String::from(" ml")},
            &Unit::Liter => {String::from(" l")},
            &Unit::Quantity => {String::from(" x")},
            &Unit::TeaSpoon => {String::from(" TL")},
            &Unit::Spoon => {String::from(" EL")},
        }
    }

}

fn new_line(s : &mut String){
    s.push_str("\n");
}