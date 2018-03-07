
use std::sync::RwLock;

use tests::mocks::RecipeDAOMock;

use recipe_bot::commands::command::RecipeCommand;

#[test]
fn details_command_test(){
    let dao = RwLock::new(Box::new(RecipeDAOMock::new()));

    //First we dont pass any arguments, we should get
    //an error for invalid input
    let args = None;

    //now we does the same with an empty vec

    //arg literal instead of number

    //right argument

    //ids > 1

    
}