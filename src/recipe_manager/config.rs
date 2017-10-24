#[derive(Debug,Clone,Hash, Eq, PartialEq)]
/// Configuration for a Database connection
pub struct DatabaseConfig{
    port: u16,
    host: String,
    user: String,
    password: Option<String>,
    database: String,
}

impl DatabaseConfig{
    pub fn new(port: u16, host: String, user: String, password: Option<String>, database: String) -> DatabaseConfig{
        DatabaseConfig{
            port: port,
            host: host,
            user: user,
            password: password,
            database: database,
        }
    }

    /// returns the port of the database
    pub fn port(&self) -> u16{
        self.port
    }

    /// Hostname/Address of the database
    pub fn host(&self) -> String{
        self.host.clone()
    }

    /// Returns the Database User
    pub fn user(&self) -> String{
        self.user.clone()
    }

    pub fn password(&self) -> Option<String>{
        match &self.password{
            &Some(ref pwd) => {
                Some(pwd.clone())
            } 
            _ => None
        }
    }

    pub fn database(&self) -> String{
        self.database.clone()
    }
}