
extern crate rocket;
//extern crate serde_json;

use std::thread;
use std::thread::JoinHandle;

use rocket::Route;
use rocket::Config;
use rocket::config::{LoggingLevel};
use rocket::State;
use rocket_contrib::json::Json;

use std::io::Error;

use recipe_manager::configuration::config::*;
use recipe_manager::services::RecipeWebService;
use recipe_manager::factory::ServiceFactory;
use recipe_manager::objects::*;

/// Struct to host the Web API. 
/// The API is build on the Rocket-Framework.
pub struct RecipeWebServiceImpl{
    config: ApplicationConfig,
    rocket_config: Config,
}

impl RecipeWebServiceImpl {
    /// Creates a new RecipeWebServiceImpl. 
     pub fn new(config: ApplicationConfig) -> RecipeWebServiceImpl{
        let rocket_config = RecipeWebServiceImpl::rocket_config(&config);
        RecipeWebServiceImpl{
            config: config,
            rocket_config: rocket_config,
        }
    }

    /// Creates a Vec of all Routes. 
    fn webscripts(&self) -> Vec<Route>{
        routes![recipes,login,add,update]
    }

    /// Translate the ApplicationConfig to the Configuration Struct
    /// of the Rocket-Framework
    fn rocket_config(config: &ApplicationConfig) -> Config{
        let web_conf = config.webservice_config();
        let mut rocket_conf: Config = match config.mode(){
            &RunningMode::Development => {Config::development().unwrap()},
            &RunningMode::Production => {Config::production().unwrap()},
        };

        rocket_conf.set_address(web_conf.address().clone()).unwrap();
        rocket_conf.set_port(web_conf.port());
        rocket_conf.set_workers(web_conf.workers());
        rocket_conf.set_log_level(LoggingLevel::Normal);
        
        //if tls is active we set the path to our cert and private key to the rocket config
        if web_conf.use_tls() {
            rocket_conf.set_tls(web_conf.certificate(),web_conf.private_key()).unwrap();
        }

        rocket_conf
    }

}

impl RecipeWebService for RecipeWebServiceImpl {

    fn start(&self) -> Result<JoinHandle<()>,Error>{

        let conf = self.rocket_config.clone();
        let webscripts = self.webscripts();
        let factory = ServiceFactory::new(self.config.clone());

        // Spawn a new Thread for our rocket webserver
        // we need the rocketConfig, our web functions/routes 
        // and we need the ServiceFactory which must be managed
        // by the rocket Framework, so that we can use the Factory 
        // in our route functions.
        let handle = thread::spawn(||{
            rocket::custom(conf, true).manage(factory).mount("/", webscripts).launch(); 
        });
        
        Ok(handle)
    }

    fn stop(&self) -> Result<(),Error>{
        // find a way to interrupt the Rocket Thread
        panic!("Oh Memories!
                Where'd you go?
                You were all I've ever known!
                How I miss yesterday!
                How'd I let it fade away?
                Don't fade away!");
    }

}

// Web API Functions

// returns all Recipes in the Database as JSON
#[get("/recipes")]
fn recipes(state: State<ServiceFactory>) -> Result<Json<Vec<Recipe>>,Error>{
    let recipes: Vec<Recipe> = state.recipe_service()?.reciptes()?;
    Ok(Json(recipes))
}

#[get("/login")]
fn login() -> String{
    unimplemented!("currently under development");
}

#[post("/add", data = "<token>")]
fn add(state: State<ServiceFactory>, token: String) -> Result<String,Error>{
    Ok(String::from("currently not implemented"))
}

#[post("/update", data = "<token>")]
fn update(token: String) -> Result<String,Error>{
    unimplemented!("currently not supported");
}