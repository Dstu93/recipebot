
extern crate postgres;
extern crate argon2rs;
extern crate rand;

use std::collections::HashMap;
use std::time::{Duration,SystemTime};

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

pub struct AuthenticationServiceImpl {
    cache: HashMap<String,Login>,
    user_service: Box<UserService + 'static>,
    cache_size: u32,
}

impl AuthenticationServiceImpl {

    /// creates new AuthenticationService with a HashMap as cache.
    /// the userservice is needed for validation and the cachesize 
    /// is the limit of logins.
    pub fn new(user_service: Box<UserService + 'static>, cache_size: u32) -> AuthenticationServiceImpl {
        // set the cache size to default 30 logins
        AuthenticationServiceImpl{
            cache: HashMap::with_capacity(30),
            user_service: user_service,
            cache_size: cache_size,
        }
    }

    /// removes all tickets from the cache that have not been used for longer than the limit.
    fn remove_tickets(map: &mut HashMap<String, Login>, limit: Duration) {
        let mut to_remove = Vec::new();
        for (key, login) in &*map {
            if login.last_used() > limit.as_secs() {
                to_remove.push(key.to_owned());
            }
        }
        for key in to_remove.iter() {
            map.remove(key);
        }
    }
}

impl AuthenticationService for AuthenticationServiceImpl {

    fn validate_ticket(&self, ticket: &String) -> bool{
        self.cache.contains_key(ticket)
    }

    fn refresh_ticket(&mut self, ticket: &String) -> bool{
        if self.cache.contains_key(ticket) {
            let login = self.cache.get_mut(ticket).unwrap();
            login.used();
            true
        } else {false}
    }

    fn login(&mut self, username: &String, password: &String) -> Result<Option<String>,Error>{

        if self.cache.len() >= self.cache_size as usize {
            // return an error to handle the cache overflow?
            println!("The Login-Cache is full, cant authenticate user [{}]", username);
            return Ok(None);
        }

        let authenticated = self.user_service.authenticate(username,password)?;
        if authenticated {
            let mut rnd = OsRng::new()?;
            let ticket: String = rnd.gen_ascii_chars().take(32).collect();
            let user = self.user_service.by_name(username)?.unwrap();
            let login = Login::new(user);
            self.cache.insert(ticket.clone(),login);
            Ok(Some(ticket))
        }
        else{
            Ok(None)
        }
    }

    fn logout(&mut self, ticket: &String) -> bool{

        //remove token from the cache
        let value = self.cache.remove(ticket);
        match value {
            Some(login) => {println!("Removed user {:?} from login cache",login.user()); true},
            None => {println!("the ticket isnt cached and is userles");false},
        }
    }

    fn get_user_from_token(&self, ticket: &String) -> Option<&User>{
        let result = self.cache.get(ticket);
        let user = match result {
            Some(login) => Some(login.user()),
            None => None,
        };
        user
    }

    fn clear_cache(&mut self){
        self.cache.clear();
    }

    fn rm_logins(&mut self,limit: Duration){
       AuthenticationServiceImpl::remove_tickets(&mut self.cache, limit);
    }
}

#[derive(Debug)]
struct Login {
    user: User,
    since: SystemTime,
    last_used: SystemTime,
}

impl Login {
    
    /// creates a new Login for the given user
    pub fn new(user: User) -> Login{
        Login{
            user: user,
            since: SystemTime::now(),
            last_used: SystemTime::now(),
        }
    }

    /// returns the duration between the past and the current systemtime as seconds.
    /// in case of an error it returns the max 2^64 
    fn between_now(earlier: SystemTime) -> u64{
        let between = earlier.duration_since(SystemTime::now());
        let seconds = match between {
            Ok(duration) => {duration.as_secs()},
            Err(_) => {2^64},
        };
        seconds
    }

    /// returns the user for this login
    pub fn user(&self) -> &User{
        &self.user
    }

    /// returns the Duration between the current time and the 
    /// time of the last used of this login.
    pub fn last_used(&self) ->  u64{
        Login::between_now(self.last_used)
    }

    /// refresh the last used field to the current systemtime
    pub fn used(&mut self){
        self.last_used = SystemTime::now();
    }

}