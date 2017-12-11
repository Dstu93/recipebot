
use std::io::Error;
use web_api::services::{UserService,AuthenticationService};
use web_api::implementations::AuthenticationServiceImpl;
use web_api::user::User;

//mock the userService

struct UserServiceMock;

impl UserService for UserServiceMock {
        /// Add a new user to the user store
    fn create(&self, name: &String, password: &String) -> Result<bool,Error>{
        Ok(true)
    }

    /// add new admin user to the user store
    fn create_admin(&self, name: &String, password: &String) -> Result<bool,Error>{
        Ok(true)
    }

    /// search user with the given id.
    fn by_id(&self, id: i32) -> Result<Option<User>,Error>{
        Ok(None)
    }

    /// search user with given name. 
    fn by_name(&self, username: &String) -> Result<Option<User>,Error>{
        if "demo_user".eq(username){
            let user = User::new(32,String::from("demo_user"),Vec::new(),"".into(),false);
            Ok(Some(user))
        } else {Ok(None)}
    }

    /// deletes the user and all informations of and for the user from the database
    fn delete(&self, id: i32) -> Result<bool, Error>{
        Ok(false)
    } 

    /// checks the pw and the user name.
    fn authenticate(&self,name: &String, password: &String) -> Result<bool,Error>{
        println!("Prüfe ob Name {:?} und password {:?} übereinstimmen", name,password);
        if "demo_user".eq(name) && "password".eq(password){
            println!("Stimmen überein.");
            Ok(true)
        } else {Ok(false)}
    }

}

#[test]
fn authenticationserviceimpl_test() {

    //non existing user
    let tom = String::from("tom");
    let tom_password = String::from("toms_password");

    //existing valid user 
    let demo_user = String::from("demo_user");
    let demo_user_password = String::from("password");

    //init our UserService mock for mocking the database access
    let user_service = Box::new(UserServiceMock);
    let mut auth_service = AuthenticationServiceImpl::new(user_service,2);

    //try to login with a non existing user
    let tom_ticket = auth_service.login(&tom, &tom_password).unwrap();
    assert_eq!(tom_ticket, None);

    //try to login with the only existing user
    let demo_ticket = auth_service.login(&demo_user,&demo_user_password).unwrap();
    assert!(demo_ticket != None);

    //validate the ticket again
    let ticket = demo_ticket.unwrap();
    let is_valid = auth_service.validate_ticket(&ticket);
    assert!(is_valid);

    //logout our demo_user and try to validate again.
    let successfull_logout = auth_service.logout(&ticket);
    let is_valid = auth_service.validate_ticket(&ticket);
    assert!(successfull_logout);
    assert!(!is_valid);

    //now we fill the cache and try to login, it should fail because the cache is full
    let first_login = auth_service.login(&demo_user, &demo_user_password).unwrap();
    assert!(first_login != None);
    let second_login = auth_service.login(&demo_user, &demo_user_password).unwrap();
    assert!(second_login != None);
    let third_login = auth_service.login(&demo_user, &demo_user_password).unwrap();
    assert_eq!(third_login, None);

    // cache is full, so we clear and try to valid the users again
    auth_service.clear_cache();

    let first_valid = auth_service.validate_ticket(&first_login.unwrap());
    assert!( !first_valid );
    let second_valid = auth_service.validate_ticket(&second_login.unwrap());
    assert!( !second_valid );

}
