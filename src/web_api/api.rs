
extern crate rocket;
//extern crate serde_json;

use std::thread;
use std::thread::JoinHandle;

use rocket::Route;
use rocket::Rocket;
use rocket::Config;
use rocket::config::{LoggingLevel};
use rocket::State;
use rocket_contrib::json::Json;

use std::io::Error;
use std::sync::Mutex;
use std::sync::RwLock;

use web_api::services::{AuthenticationService,UserService};
use web_api::implementations::{UserServiceImpl,AuthenticationServiceImpl};

use recipe_manager::configuration::config::*;
use recipe_manager::services::{RecipeWebService,RecipeDAO};
use recipe_manager::factory::ServiceFactory;
use recipe_manager::objects::*;


use web_api::webscripts;
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
        //we need to create our needed DAOs and Services
        let recipe_dao = ServiceFactory::recipe_service(&self.config.database_config())?;
        let userservice = Box::new(UserServiceImpl::new(&self.config.database_config()));
        let auth_service = Box::new(AuthenticationServiceImpl::new(userservice,30));

        //now we build the rocket instance, TODO


        //we need a factory here to create the needed things

        // Spawn a new Thread for our rocket webserver
        // we need the rocketConfig, our web functions/routes 
        // and we need the ServiceFactory which must be managed
        // by the rocket Framework, so that we can use the Factory 
        // in our route functions.
        let handle = thread::spawn(||{
            //rocket::custom(conf, true).manage(recipe_dao).mount("/", webscripts).launch(); 
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

pub fn rocket(recipe_dao: Box<RecipeDAO + Send + Sync>, auth_service: Box<AuthenticationService + Send>, rocket_config: Config) -> Rocket {
        let authentication_service = Mutex::new(auth_service);
        let recipeservice = RwLock::new(recipe_dao);
        let rocket = rocket::custom(rocket_config, true)
            .manage(recipeservice)
            .manage(authentication_service)
            .mount("/", webscripts::build());
        
        rocket
    }