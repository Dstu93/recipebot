
use std::io::Error;
use std::time::Duration;

use web_api::user::User;

/// trait for creating or requesting users from some storage
pub trait UserService {
    
    /// Add a new user to the user store
    fn create(&self, name: &String, password: &String) -> Result<bool,Error>;

    /// add new admin user to the user store
    fn create_admin(&self, name: &String, password: &String) -> Result<bool,Error>;

    /// search user with the given id.
    fn by_id(&self, id: i32) -> Result<Option<User>,Error>;

    /// search user with given name. 
    fn by_name(&self, username: &String) -> Result<Option<User>,Error>;

    /// deletes the user and all informations of and for the user from the database
    fn delete(&self, id: i32) -> Result<bool, Error>; 

    /// checks the pw and the user name.
    fn authenticate(&self,name: &String, password: &String) -> Result<bool,Error>;
}

/// service for Authentication of user logins and tickets
pub trait AuthenticationService{

    /// validates a ticket, returns true if ticket is valid and false if not.
    /// also the ticket gets refreshed.
    fn validate_ticket(&self, ticket: &String) -> bool;

    /// refreshs the given ticket and set the last_used flag to the current systemtime.
    /// returns true if success and false if ticket is not cached and valid.
    fn refresh_ticket(&mut self, ticket: &String) -> bool;

    /// fn for user login. if data is valid it returns a ticket. if not valid it returns None.
    /// can return an error if the database or other io access gone wrong.
    fn login(&mut self, username: &String, password: &String) -> Result<Option<String>,Error>;

    /// invalidate the ticket and logout the user
    fn logout(&mut self, ticket: &String) -> bool;

    /// returns the user behind the token
    fn get_user_from_token(&self, ticket: &String) -> Option<&User>;

    /// clears the login cache and logsout all users
    fn clear_cache(&mut self);

    /// removes all tickets that are older than the limit.
    /// every time a ticket gets used for authentication it gets refreshed.
    fn rm_logins(&mut self, limit: Duration);
}