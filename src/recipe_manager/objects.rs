


///Object to represent a recipe
#[derive(Debug,Clone,PartialEq)]
pub struct Recipe{
    id : u32,
    name : String,
    descr : String,
    ingredients : Vec<Ingredient>
}

#[allow(dead_code)]
impl Recipe{
    ///Creates a new Recipe. Ingredients must add after creation.
    pub fn new(id : u32, name : String, descr : String) -> Recipe{
        Recipe{
            id : id,
            name : name,
            descr : descr,
            ingredients : Vec::new(),
        }
    }

    ///Adds an Ingredient to the Recipe
    pub fn add(&mut self, ingredient : Ingredient){
        &self.ingredients.push(ingredient);
    }
}

//---------------------------------------------------------------------------

///represents an Ingredient of a Recipe
#[derive(Debug,Clone,PartialEq)]
pub struct Ingredient{
    name : String,
    unit : Unit,
    quantity : f32,
}

#[allow(dead_code)]
impl Ingredient{
    ///Creates a new Ingredient.
    pub fn new(name : String, unit : Unit, quantity : f32) -> Ingredient{
        Ingredient{
            name : name,
            unit : unit,
            quantity : quantity,
        }
    }

    ///Gets the name of the Ingredient.
    pub fn name(&self) -> &String{
        &self.name
    }

    ///returns the messerument Unit of the Ingredient.
    pub fn unit(&self) -> Unit{
        self.unit
    }

    /// The Quantity of the Ingredient and of the Unit
    pub fn quantity(&self) -> f32{
        self.quantity
    }
}

//---------------------------------------------------------------------------

///Enum of Unit Types of Ingredients
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