
use recipe_manager::traits::RecipeFmt;
use recipe_manager::utils::new_line;

///Object to represent a recipe
#[derive(Debug,Clone,PartialEq)]
pub struct Recipe{
    id: i32,
    name: String,
    descr: String,
    persons: i16,
    path: String,
    ingredients: Vec<Ingredient>,
}

#[allow(dead_code)]
impl Recipe{
    
    ///Creates a new Recipe. Ingredients must add after creation.
    pub fn new(id: i32, name: String, descr: String, persons: i16) -> Recipe{
        Recipe::with_ingredients(id,name,descr,persons,Vec::new())
    }

    /// Creates a new recipe with a given vector of ingredients
    pub fn with_ingredients(id: i32, name: String, descr: String, persons: i16, ingr: Vec<Ingredient>) -> Recipe{
        Recipe{
            id : id,
            persons: persons, 
            name : name,
            descr : descr,
            path: String::from("/"),
            ingredients : ingr,
        }
    }

    /// Returns the Id of the Recipe
    pub fn id(&self) -> i32{
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

    /// Number of persons for which this recipe is intended
    pub fn persons(&self) -> i16{
        self.persons
    }

    /// Path of the recipes. Its meant as a virtual Directory Path
    /// to order recipes in categorys. 
    pub fn path(&self) -> String{
        self.path.clone()
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
#[derive(Debug,Clone,PartialEq,Serialize, Deserialize)]
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
#[derive(Debug,Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
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

/* 
#[derive(Debug,Clone, Hash, Eq, PartialEq)]
pub struct Node<T>{
    name: String,
    id: u32,
    values: Vec<T>,
    childs: Vec<Node<T>>,
}

impl<T> Node<T>{

    /// creates a new Node, the id must be unique.
    /// if the id is not unique, it can cause collisions in a Tree
    pub fn new(node_id: u32, name: String) -> Node<T>{
        Node{
            name: name,
            id: node_id,
            values: Vec::new(),
            childs: Vec::new(),
        }
    }


}

impl<T> RecipeFmt for Node<T>{
    fn display(&self) -> String{
        unimplemented!("No formatting implemented for Node")
    }
}

// Tree for nodes

pub struct Tree<T>{
    root_node: Node<T>,
    id_cache: HashMap<u32,Node<T>>,
    name_id_cache: HashMap<String,u32>,
} 
*/