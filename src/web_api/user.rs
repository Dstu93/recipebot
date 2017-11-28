
#[derive(Debug)]
/// struct to represent an user
pub struct User {
    id: i32,
    name: String,
    pw_hash: Vec<u8>,
    salt: String,
    is_admin: bool,
}

impl User {

    /// Creates a new User with given name and user id
    pub fn new(user_id: i32,name: String, password_hash: Vec<u8>, salt: String, admin: bool) -> User{
        User{
            id: user_id,
            name: name,
            pw_hash: password_hash,
            salt: salt,
            is_admin: admin,
        }
    }

    /// returns the user id
    pub fn id(&self) -> i32{
        self.id
    }

    /// returns the user name
    pub fn name(&self) -> &String{
        &self.name
    }

    /// returns the password hash as byte vec
    pub fn password(&self) -> &Vec<u8>{
        &self.pw_hash
    }

    /// returns the salt of the password hash
    pub fn salt(&self) -> &String{
        &self.salt
    }

    /// returns true if the user is an admin
    pub fn is_admin(&self) -> bool{
        self.is_admin
    }
}