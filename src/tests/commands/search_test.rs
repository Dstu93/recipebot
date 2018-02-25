
use tests::mocks::RecipeDAOMock;
use recipe_bot::commands::search::SearchCmd;
use recipe_bot::commands::error::CmdError;
use std::sync::RwLock;

#[test]
fn search_cmd_test(){

    let dao = Box::new(RecipeDAOMock::new());
    let cmd = SearchCmd::new(RwLock::new(dao));

    let result = cmd.exec_cmd(None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(),CmdError::NoArguments);

    let mut recipes = Vec::new();
    let result = cmd.exec_cmd(Some(recipes));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(),CmdError::NoArguments);

    recipes = Vec::new();
    recipes.push("NotAValidRecipeName");
    let result = cmd.exec_cmd(Some(recipes));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(),CmdError::NotFound);

    recipes = Vec::new();
    recipes.push("Strawberry Cake");
    let result = cmd.exec_cmd(Some(recipes));
    assert!(result.is_ok());
    println!("Cmd Answer: {}",result.unwrap());

    //fix mock..., mock does not make a like request
    //maybe starts with?
    recipes = Vec::new();
    recipes.push("Strawberry");
    recipes.push("Schoc");
    let result = cmd.exec_cmd(Some(recipes));
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 0);
}