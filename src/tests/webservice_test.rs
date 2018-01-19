
use std::io::Error;
use std::sync::Mutex;

use tests::mocks::{RecipeDAOMock,UserServiceMock};

use rocket::config::{Config,Environment};
use rocket::local::Client;
use rocket::http::{Status,ContentType};
use rocket::http::Cookie;

use rocket::http::Header;

use serde_json;

use web_api::services::{UserService,AuthenticationService};
use web_api::implementations::AuthenticationServiceImpl;
use web_api::user::User;
use web_api::api::rocket;

use recipe_manager::services::RecipeDAO;
use recipe_manager::objects::Recipe;

//need an Function to build our rocket instance

#[test]
fn webscripts_test() {

    //Setup our Rocket test environment
    let user_service = Box::new(UserServiceMock);
    let authentication_service = Box::new(AuthenticationServiceImpl::new(user_service,30));
    let recipe_dao = Box::new(RecipeDAOMock::new());
    let rocket_config = Config::build(Environment::Development).unwrap();
    let rocket = rocket(recipe_dao,authentication_service,rocket_config);
    let client = Client::new(rocket).expect("could not host local rocket");
    
    // get Request to check if we get our Recipes in the "Database"
    let mut resp = client.get("/recipes").dispatch();
    let json = resp.body_string();
    assert!(json.is_some());
    let responsed_recipes: Vec<Recipe> = serde_json::from_str(&json.unwrap()).unwrap();
    let mut test_recipes = Vec::new();
    RecipeDAOMock::fill_with_recipes(&mut test_recipes);
    assert_eq!(test_recipes,responsed_recipes);

    // post /add test, we test if we can add a recipe without authorization
    let mut req = client.post("/add");
    let new_recipe = Recipe::new(2,"Air".into(),"just breath".into(),1);
    let mut new_recipes = Vec::new();
    new_recipes.push(new_recipe.clone());
    req.add_header(ContentType::JSON);
    req.set_body(serde_json::to_string(&new_recipes).unwrap());
    let mut resp = req.dispatch();
    assert_eq!(resp.status(),Status::Forbidden);

    // now we try to login with a malformed header, we should get a 400 back
    let mut req = client.post("/login");
    let authorization = Header::new("Authorization", "Basic falseheadercontentuwahduawh");
    let mut resp = req.header(authorization).dispatch();
    assert_eq!(resp.status(), Status::BadRequest);

    // try to login again with false username and password, false_user:badpassword
    let mut req = client.post("/login");
    let authorization = Header::new("Authorization", "Basic ZmFsc2VfdXNlcjpiYWRwYXNzd29yZA==");
    let mut resp = req.header(authorization).dispatch();
    assert_eq!(resp.status(), Status::Forbidden);

    // try to login again with false username and empty password, false_user:
    let mut req = client.post("/login");
    let authorization = Header::new("Authorization", "Basic ZmFsc2VfdXNlcjo=");
    let mut resp = req.header(authorization).dispatch();
    assert_eq!(resp.status(), Status::Forbidden);

    //try to login with a correct username and password, demo_user:password
    let mut req = client.post("/login");
    let authorization = Header::new("Authorization", "Basic ZGVtb191c2VyOnBhc3N3b3Jk");
    let mut resp = req.header(authorization).dispatch();
    let cookie_value = String::from(resp.headers().get_one("Set-Cookie").unwrap());
    assert_eq!(resp.status(), Status::Accepted);

    // our authorization cookie
    let cookie = Cookie::parse(cookie_value).unwrap();

    let mut req = client.post("/add").cookie(cookie.clone());
    req.set_body(serde_json::to_string(&new_recipes).unwrap());
    req.add_header(ContentType::JSON);
    let mut resp = req.dispatch();
    assert_eq!(resp.status(),Status::Accepted);

    // We'll renew our ticket so we don't get locked out automatically.
    let mut resp = client.post("/refresh").cookie(cookie.clone()).dispatch();
    assert_eq!(resp.status(),Status::Accepted);

    // After we have sent a new recipe to the server,
    // we now check if it has been stored in the database and we can retrieve it again.
    let mut resp = client.get("/recipes").dispatch();
    let json = resp.body_string();
    assert!(json.is_some());
    let responded_recipes: Vec<Recipe> = serde_json::from_str(&json.unwrap()).unwrap();
    assert!(responded_recipes.contains(&new_recipe));

    // we try now to change the description of our newly added recipe.
    let updated_recipe = Recipe::new(2,"Air".into(),"just breath freshly air".into(),1);
    let mut updated_recipes = Vec::new();
    updated_recipes.push(updated_recipe.clone());
    let mut req = client.post("/update").cookie(cookie.clone());
    req.set_body(serde_json::to_string(&updated_recipes).unwrap());
    req.add_header(ContentType::JSON);
    let mut resp = req.dispatch();
    assert_eq!(resp.status(),Status::Accepted);

    // now we want to remove our updated recipe from the database
    let mut req = client.post("/delete").cookie(cookie.clone());
    req.set_body(serde_json::to_string(&updated_recipes).unwrap());
    req.add_header(ContentType::JSON);
    let mut resp = req.dispatch();
    assert_eq!(resp.status(),Status::Accepted);

    // prove the deletion
    let mut resp = client.get("/recipes").dispatch();
    let json = resp.body_string();
    assert!(json.is_some());
    let responded_recipes: Vec<Recipe> = serde_json::from_str(&json.unwrap()).unwrap();
    assert!(!responded_recipes.contains(&updated_recipe));

}