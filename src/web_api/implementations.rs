
extern crate postgres;
extern crate argon2rs;
extern crate rand;

use self::postgres::params::{Host,Builder};
use self::postgres::{TlsMode,Connection};
use self::rand::os::OsRng;
use self::rand::Rng;

use std::io::Error;
use recipe_manager::configuration::config::DatabaseConfig;
use web_api::services::*;
use web_api::user::User;

/// Implementation of the UserService, uses postgreSQL
pub struct UserServiceImpl {
    db_config: DatabaseConfig, 
}

impl UserServiceImpl {
    
    /// Creates a new UserService
    pub fn new(config: &DatabaseConfig) -> UserServiceImpl{
        UserServiceImpl{
            db_config: config.clone(),
        }
    } 

    /// builds a Connection to the postgreSQL Database
    fn connect(&self) -> Result<Connection, Error>{
        let mut builder = Builder::new();
        builder.port(self.db_config.port());
        builder.user(&self.db_config.user(),Some(&self.db_config.password().unwrap()));
        builder.database(&self.db_config.database());
        let params = builder.build(Host::Tcp(self.db_config.host()));

        let conn = Connection::connect(params, TlsMode::None)?;
        Ok(conn)
    }

    /// this function adds a new user to the database
    /// the password gets argon2 hashed with a use specific random salt.
    fn create_user(&self,user_name: &String, password: &String,is_admin: bool) -> Result<(),Error>{
        //we use the os random generator for generating a 32 byte sized ascii string
        //and use it as salt for our password hash
        let mut rand = OsRng::new()?;
        let salt: String = rand.gen_ascii_chars().take(32).collect();
        let pw_hash = argon2rs::argon2i_simple(password, &salt).to_vec();
        let conn = self.connect()?;
        conn.execute("INSERT INTO users (username, password,salt,admin) VALUES ($1, $2, $3, $4)",
                 &[user_name, &pw_hash,&salt,&is_admin])?;
        Ok(())
    }
}

impl UserService for UserServiceImpl {
    
    fn create(&self, name: &String, password: &String) -> Result<bool,Error>{
        self.create_user(name, password,false)?;
        Ok(true)
    }

    fn create_admin(&self, name: &String, password: &String) -> Result<bool,Error>{
        self.create_user(name, password,true)?;
        Ok(true)
    }

    fn by_id(&self, id: i32) -> Result<Option<User>,Error>{
        let conn = self.connect()?;
        for row in &conn.query("SELECT username,password,salt,admin FROM users WHERE id = $1 ", &[&id])? {
            let user_id: i32 = row.get(0);
            let username: String = row.get(1);
            let password: Vec<u8> = row.get(2);
            let salt: String = row.get(3);
            let is_admin: bool = row.get(4);
            let user = User::new(user_id,username,password,salt,is_admin);
            return Ok(Some(user));
        }
        Ok(None)
    }
 
    fn by_name(&self, name: &String) -> Result<Option<User>,Error>{
        let conn = self.connect()?;
        for row in &conn.query("SELECT id,password,salt,admin FROM users WHERE username = $1 ", &[name])? {
            let user_id: i32 = row.get(0);
            let password: Vec<u8> = row.get(1);
            let salt: String = row.get(2);
            let is_admin: bool = row.get(3);
            let user = User::new(user_id,name.clone(),password,salt,is_admin);
            return Ok(Some(user));
        }
        Ok(None)
    }

    fn delete(&self, id: i32) -> Result<bool, Error>{
        let conn = self.connect()?;
        let result = conn.execute("DELETE FROM users WHERE id = $1",&[&id])?;
        //log out the number of affected rows 
        Ok(true)
    }

    fn authenticate(&self,name: &String, password: &String) -> Result<bool,Error>{        
        let name = self.by_name(name)?;
        match name {
            None => {Ok(false)}
            Some(user) => { 
                let user_salt = user.salt();
                let password_hash = argon2rs::argon2i_simple(password, user_salt).to_vec();
                Ok(password_hash.eq(user.password()))
            }
        }
    }
}

//TODO implement AuthenticationService mit login cache 
//inkl. einem Thread der den Cache im gewissen Abstand 
//prüft und leert