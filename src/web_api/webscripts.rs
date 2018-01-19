
use rocket::Route;
use rocket::State;
use rocket_contrib::json::Json;
use rocket::http::{Cookies,Cookie,Status};
use rocket::Request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::response::Failure;
use rocket::response::status::Accepted;
use rocket;

use std::io::Error;
use std::sync::Mutex;
use std::sync::RwLock;

use recipe_manager::factory::ServiceFactory;
use recipe_manager::services::RecipeDAO;
use recipe_manager::objects::*;
use recipe_manager::utils;

use web_api::services::AuthenticationService;

/// creates a Vec of all webscripts to host.
pub fn build() -> Vec<Route>{
    routes![recipes,login,logout,refresh,add, update, delete]
}

// Web API Functions

/// returns all Recipes in the Database as JSON//check if login is successfull
#[get("/recipes")]
fn recipes(state: State<RwLock<Box<RecipeDAO + Send + Sync>>>) -> Result<Json<Vec<Recipe>>,Error>{
    let recipes: Vec<Recipe> = state.read().unwrap().reciptes()?;
    Ok(Json(recipes))
}


//Brauche username und password
#[post("/login")]
fn login(auth: State<Mutex<Box<AuthenticationService + Send>>>, mut cookies: Cookies, login: Login) -> Result<Accepted<String>,Failure>{

    let username = login.username();
    let password = login.pwd();

    match auth.lock().as_mut().unwrap().login(&username,&password) {
        Ok(ticket) => {
            if ticket.is_some() {
                // TODO expiration date and https only flags etc.
                let cookie = Cookie::build("ticket", ticket.unwrap())
                    .secure(true)
                    .domain("localhost")
                    .http_only(true)
                    .finish();
                cookies.add(cookie);
                Ok(Accepted(Some(String::from("success"))))
            }
            else {
                Err(Failure::from(Status::Forbidden))
            }
        },
        Err(_) => {
            //TOO logg error
            Err(Failure::from(Status::InternalServerError))
        },
    }
}

#[post("/logout")]
fn logout(auth: State<Mutex<Box<AuthenticationService + Send>>>, mut cookies: Cookies){
    //clear the ticket from the cache and unset the cookie.
    let cookie = cookies.get_private("ticket");
    if cookie.is_some() {
        let cookie = cookie.unwrap();
        let ticket = String::from(cookie.value());
        //looks ugly, is there a better way?
        auth.lock().as_mut().unwrap().logout(&ticket);
        cookies.remove(cookie);
    } 
    else {
    }
}

#[post("/refresh")]
fn refresh(auth: State<Mutex<Box<AuthenticationService + Send>>>, mut cookies: Cookies) -> Result<Accepted<String>,Failure>{
    // print all cookies.
    for c in cookies.iter(){
        println!("Cookie: {}, Value: {}",c.name(),c.value());
    }

    let cookie = cookies.get("ticket");
    if cookie.is_some() {
        let cookie = cookie.unwrap();
        let ticket = String::from(cookie.value());
        println!("Refresh Ticket: {}", &ticket);
        let success = auth.lock().as_mut().unwrap().refresh_ticket(&ticket);
        if success {
            Ok(Accepted(Some(String::from("success"))))
        } else {Err(Failure::from(Status::Forbidden))}
    } else {Err(Failure::from(Status::BadRequest))}
}

#[post("/add", data = "<recipes>")]
fn add(dao: State<RwLock<Box<RecipeDAO + Send + Sync>>>, recipes: Json<Vec<Recipe>>,auth: State<Mutex<Box<AuthenticationService + Send>>>, mut cookies: Cookies) -> Result<Accepted<String>,Failure>{

    let ticket_cookie = cookies.get("ticket");
    if ticket_cookie.is_none() {
        return Err(Failure::from(Status::Forbidden));
    } else {
        let ticket = String::from(ticket_cookie.unwrap().value());
        let success = auth.lock().as_mut().unwrap().refresh_ticket(&ticket);
        if success {
            let mut recipe_dao = dao.write().unwrap();
            for recipe in recipes.iter() {
                match recipe_dao.add(&recipe){
                    Ok(_) => {},
                    Err(_) => {return Err(Failure::from(Status::InternalServerError))},
                };
            }
            Ok(Accepted(Some(String::from("success"))))
        } else { Err(Failure::from(Status::Forbidden)) }
    }

}

#[post("/update", data = "<recipes>")]
fn update(dao: State<RwLock<Box<RecipeDAO + Send + Sync>>>, recipes: Json<Vec<Recipe>>,auth: State<Mutex<Box<AuthenticationService + Send>>>, mut cookies: Cookies) -> Result<Accepted<String>,Failure>{

    let ticket_cookie = cookies.get("ticket");
    if ticket_cookie.is_none() {
        return Err(Failure::from(Status::Forbidden));
    } else {
        let ticket = String::from(ticket_cookie.unwrap().value());
        let success = auth.lock().as_mut().unwrap().refresh_ticket(&ticket);
        if success {
            let mut recipe_dao = dao.write().unwrap();
            for recipe in recipes.iter() {
                match recipe_dao.update(&recipe){
                    Ok(_) => {},
                    Err(_) => {return Err(Failure::from(Status::InternalServerError))},
                };
            }
            Ok(Accepted(Some(String::from("success"))))
        } else { Err(Failure::from(Status::Forbidden)) }
    }
}

#[post("/delete", data = "<recipes>")]
fn delete(dao: State<RwLock<Box<RecipeDAO + Send + Sync>>>, recipes: Json<Vec<Recipe>>,auth: State<Mutex<Box<AuthenticationService + Send>>>, mut cookies: Cookies) -> Result<Accepted<String>,Failure>{

    let ticket_cookie = cookies.get("ticket");
    if ticket_cookie.is_none() {
        return Err(Failure::from(Status::Forbidden));
    } else {
        let ticket = String::from(ticket_cookie.unwrap().value());
        let success = auth.lock().as_mut().unwrap().refresh_ticket(&ticket);
        if success {
            let mut recipe_dao = dao.write().unwrap();
            for recipe in recipes.iter() {
                let result = recipe_dao.delete(&recipe);
                match result {
                    Ok(_) => {},
                    Err(_) => { return Err(Failure::from(Status::InternalServerError))},
                };
            }
            Ok(Accepted(Some(String::from("success"))))
        } else { Err(Failure::from(Status::Forbidden)) }
    }
}

#[derive(Debug)]
struct Login{
    username: String,
    pwd: String,
}

impl Login {
    /// returns the username
    pub fn username(&self) -> &String{
        &self.username
    }
    /// returns the password
    pub fn pwd(&self) -> &String{
        &self.pwd
    }
}

impl <'a,'r>FromRequest<'a,'r> for Login {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let header_values: Vec<&str> = request.headers().get("Authorization").collect();
        
        if header_values.len() != 1 {
            println!("Content Laenge des Heaers zu lang");
            return rocket::Outcome::Failure((Status::new(400,"invalid authorization header"),()));
        }

        let content = header_values[0];
        if !content.starts_with("Basic ") {
            println!("Header Content startet nicht mit Basic");
            return rocket::Outcome::Failure((Status::new(400,"only http basic authenticaion is supported"),()));
        }

        // split the content to get the base64 string after the "basic " 
        let base64 = content.split_at(6).1;
        // decode base64
        let decoded = match utils::base64_decode(&base64.to_string()) {
            Ok(bytes) => {
                let res = String::from_utf8(bytes);
                if res.is_err() {
                    println!("konnte String nicht von Bytes parsen");
                    return rocket::Outcome::Failure((Status::BadRequest,()));
                }
                else {res.unwrap()}
            },
            Err(e) => {println!("Fehler beim Parsen: {:#?}", e); return rocket::Outcome::Failure((Status::BadRequest,()));},
        };
        // split the username and the password at the ":" seperator
        let mut split = decoded.split(":");
        // before unwrap, better check the option and throw a 400 if its None?
        let username: String = split.next().unwrap().into();
        let pwd: String = split.next().unwrap().into();

        return rocket::Outcome::Success(Login{username: username,pwd: pwd,});
    }
}