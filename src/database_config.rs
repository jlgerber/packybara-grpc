use num_cpus;
use std::convert::From;

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub pool_procs: u16,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 5432,
            user: "postgres".into(),
            password: "example".into(),
            database: "packrat".into(),
            pool_procs: num_cpus::get() as u16,
        }
    }
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host<I>(mut self, host: I) -> Self
    where
        I: Into<String>,
    {
        self.host = host.into();
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = port.into();
        self
    }

    pub fn user<I>(mut self, user: I) -> Self
    where
        I: Into<String>,
    {
        self.user = user.into();
        self
    }

    pub fn password<I>(mut self, password: I) -> Self
    where
        I: Into<String>,
    {
        self.password = password.into();
        self
    }

    pub fn database<I>(mut self, database: I) -> Self
    where
        I: Into<String>,
    {
        self.database = database.into();
        self
    }
    pub fn pool_procs(mut self, procs: u16) -> Self {
        self.pool_procs = procs;
        self
    }
    pub fn to_postgres_config(&self) -> tokio_postgres::Config {
        let mut pg_config = tokio_postgres::Config::new();
        pg_config.user(&self.user);
        pg_config.dbname(&self.database);
        pg_config.password(&self.password);
        pg_config.port(self.port);
        pg_config.host(&self.host);
        pg_config
    }
}

impl From<DatabaseConfig> for tokio_postgres::Config {
    fn from(config: DatabaseConfig) -> Self {
        let mut pg_config = tokio_postgres::Config::new();
        pg_config.user(&config.user);
        pg_config.dbname(&config.database);
        pg_config.password(config.password);
        pg_config.port(config.port);
        pg_config.host(&config.host);
        pg_config
    }
}
