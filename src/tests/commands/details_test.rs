
use std::sync::RwLock;

use tests::mocks::RecipeDAOMock;

use recipe_bot::commands::command::RecipeCommand;
use recipe_bot::commands::error::CmdError;
use recipe_bot::commands::details::DetailsCommand;

#[test]
fn details_command_test(){
    let dao = Box::new(RecipeDAOMock::new());
    let cmd = DetailsCommand::new(RwLock::new(dao));

    //First we do not pass any arguments, we should get
    //an error for invalid input
    let args = None;
    let result = cmd.execute_cmd(args);
    assert_eq!(result.unwrap_err(),CmdError::NoArguments);

    //now we does the same with an empty vec
    let args = Some(Vec::new());
    let result = cmd.execute_cmd(args);
    assert_eq!(result.unwrap_err(),CmdError::NoArguments);

    //arg literal instead of number
    let args = Some(vec!["n"]);
    let result = cmd.execute_cmd(args);
    assert_eq!(result.unwrap_err(),CmdError::InvalidInput);

    //right argument
    let args = Some(vec!["0"]);
    let result = cmd.execute_cmd(args);
    assert!(result.is_ok());
    //TODO validate output string.

    //ids > 1
    let args = Some(vec!["0","1"]);
    let result = cmd.execute_cmd(args);
    assert!(result.is_ok());
    //TODO validate string

}