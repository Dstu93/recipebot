
extern crate serde_json;

use recipe_manager::configuration::config::DatabaseConfig;

use std::io::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

    /// loads the base Configuration of this Application
pub fn database_config() -> Result<DatabaseConfig, Error>{
        
    let mut json = String::new();
    let config_path = Path::new("config.json");

    let mut file: File = match File::open(&config_path){
        Ok(file) => {file},
        Err(error) => {panic!("Could not load config: {}", error)}
    };

    //read from config File to the json String
    file.read_to_string(&mut json).expect("could not read from config file");
    let config: DatabaseConfig = serde_json::from_str(&json)?;
    Ok(config)
}

pub fn create_default_config() -> Result<(),Error>{
    let config = DatabaseConfig::new(5432,"localhost".into(),"username".into(),None,"databaseName".into());
    let json = serde_json::to_string(&config)?;
    
    let config_path = Path::new("config.json");
    let mut file: File = match File::create(&config_path){
        Err(error) => {panic!("could not create Config.json {:?}",error)},
        Ok(file) => file,
    };
    
    file.write_all(json.as_bytes())?;
    Ok(())
}