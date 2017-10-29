
#[derive(Debug,Clone,Hash, Eq, PartialEq, Serialize, Deserialize)]
/// Main Configuration struct.
pub struct ApplicationConfig{
    api_key: String,
    telegrambot: bool,
    webservice: bool,
    webservice_config: WebServiceConfig,
    database_config: DatabaseConfig,
}

impl ApplicationConfig {
    
    pub fn new(api_key: String, telegrambot: bool, webservice: bool,webservice_config: WebServiceConfig,database_config: DatabaseConfig) -> ApplicationConfig{
        ApplicationConfig{
            telegrambot: telegrambot,
            webservice: webservice,
            api_key: api_key,
            webservice_config: webservice_config,
            database_config: database_config,
        }
    }

    /// returns the telegram bot api key
    pub fn api_key(&self) -> String{
        self.api_key.clone()
    }

    /// returns the DatabaseConfig for the database connection settings
    pub fn database_config(&self) -> DatabaseConfig{
        self.database_config.clone()
    }
}


#[derive(Debug,Clone,Hash, Eq, PartialEq, Serialize, Deserialize)]
/// Config for the WebService.
pub struct WebServiceConfig{
    tls: bool,
    address: String,
    port: u16,
}

impl WebServiceConfig{
    
    pub fn new(tls: bool, address: String, port: u16) -> WebServiceConfig{
        WebServiceConfig{
            tls: tls,
            address: address,
            port: port,
        }
    }

    pub fn use_tls(&self) -> bool{
        self.tls
    }

    pub fn address(&self) -> String{
        self.address.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug,Clone,Hash, Eq, PartialEq, Serialize, Deserialize)]
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