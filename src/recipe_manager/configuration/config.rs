
#[derive(Debug,Clone,Hash, Eq, PartialEq, Serialize, Deserialize)]
/// Main Configuration struct.
pub struct ApplicationConfig{
    running_mode: RunningMode,
    telegram_api_key: String,
    telegrambot_active: bool,
    webservice_active: bool,
    webservice_config: WebServiceConfig,
    database_config: DatabaseConfig,
}

impl ApplicationConfig {
    
    pub fn new(mode: RunningMode,api_key: String, telegrambot: bool, webservice: bool,webservice_config: WebServiceConfig,
        database_config: DatabaseConfig) -> ApplicationConfig{

        ApplicationConfig{
            running_mode: mode,
            telegrambot_active: telegrambot,
            webservice_active: webservice,
            telegram_api_key: api_key,
            webservice_config: webservice_config,
            database_config: database_config,
        }
    }

    pub fn mode(&self) -> &RunningMode{
        &self.running_mode
    }

    /// returns the telegram bot api key
    pub fn api_key(&self) -> String{
        self.telegram_api_key.clone()
    }

    /// returns the DatabaseConfig for the database connection settings
    pub fn database_config(&self) -> &DatabaseConfig{
        &self.database_config
    }

    /// returns the webservice Config
    pub fn webservice_config(&self) -> &WebServiceConfig{
        &self.webservice_config
    }
}


#[derive(Debug,Clone,Hash, Eq, PartialEq, Serialize, Deserialize)]
/// Config for the WebService.
pub struct WebServiceConfig{
    tls: bool,
    cert: String,
    priv_key: String,
    address: String,
    port: u16,
    workers: u16,
}

impl WebServiceConfig{
    
    pub fn new(tls: bool,cert: String, private_key: String, address: String, port: u16, workers: u16) -> WebServiceConfig{
        WebServiceConfig{
            tls: tls,
            cert: cert,
            priv_key: private_key,
            address: address,
            port: port,
            workers: workers,
        }
    }
    
    pub fn use_tls(&self) -> bool{
        self.tls
    }

    /// returns the path of the certificate
    pub fn certificate(&self) -> &String{
        &self.cert
    }

    /// the private key
    pub fn private_key(&self) -> &String{
        &self.priv_key
    }

    /// The address which should listen on
    pub fn address(&self) -> &String{
        &self.address
    }

    /// Port Number
    pub fn port(&self) -> u16 {
        self.port
    }

    /// The number of Worker Threads
    pub fn workers(&self) -> u16{
        self.workers
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

/// Enum of all possible modes in which the program can run. 
/// For example, in development or production mode
#[derive(Debug,Clone,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub enum RunningMode {
    Development,
    Production,
}