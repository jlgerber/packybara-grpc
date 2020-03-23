//! Configure the server, setting the postgres database and pool paramters
use num_cpus;
use std::convert::From;

#[derive(Debug, PartialEq, Eq)]
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
            pool_procs: 1,
        }
    }
}

impl DatabaseConfig {
    /// new up a DatabaseConfig instance
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the host name using any type that can be conferted into a String
    pub fn host<I>(mut self, host: I) -> Self
    where
        I: Into<String>,
    {
        self.host = host.into();
        self
    }
    /// Set the host name using an Option wrapped type implementing Into String
    pub fn host_opt<I>(mut self, host: Option<I>) -> Self
    where
        I: Into<String>,
    {
        if let Some(val) = host {
            self.host = val.into();
        }
        self
    }
    /// Set the port number
    pub fn port(mut self, port: u16) -> Self {
        self.port = port.into();
        self
    }
    /// Set the port number using an option wrapped u16
    pub fn port_opt(mut self, port: Option<u16>) -> Self {
        if let Some(val) = port {
            self.port = val;
        }
        self
    }

    /// Set the user using any type that implements Into String
    pub fn user<I>(mut self, user: I) -> Self
    where
        I: Into<String>,
    {
        self.user = user.into();
        self
    }

    /// Set the user via an option wrapped string
    pub fn user_opt<I>(mut self, user: Option<I>) -> Self
    where
        I: Into<String>,
    {
        if let Some(val) = user {
            self.user = val.into();
        }
        self
    }

    /// set the password given any type which implements Into String
    pub fn password<I>(mut self, password: I) -> Self
    where
        I: Into<String>,
    {
        self.password = password.into();
        self
    }

    /// Set the password using an option wrapped type implementing Into String
    pub fn password_opt<I>(mut self, password: Option<I>) -> Self
    where
        I: Into<String>,
    {
        if let Some(val) = password {
            self.password = val.into();
        }
        self
    }

    /// Set the database name using any type which implements Into String
    pub fn database<I>(mut self, database: I) -> Self
    where
        I: Into<String>,
    {
        self.database = database.into();
        self
    }
    /// Set teh database name using an Option wrapped type implementing Into String
    pub fn database_opt<I>(mut self, database: Option<I>) -> Self
    where
        I: Into<String>,
    {
        if let Some(val) = database {
            self.database = val.into();
        }
        self
    }
    /// Set the number of procs in the pool
    pub fn pool_procs(mut self, procs: u16) -> Self {
        self.pool_procs = procs;
        self
    }

    /// Set the number of procs as an Option<u16>. If None, we use
    /// the number of procs on the machine.
    pub fn pool_procs_opt(mut self, pool_procs: Option<u16>) -> Self {
        if let Some(val) = pool_procs {
            self.pool_procs = val;
        } else {
            self.pool_procs = num_cpus::get() as u16;
        }
        self
    }

    /// Generagte a tokio postgres config instance
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_initialization_works() {
        let config = DatabaseConfig::new();
        let expected = DatabaseConfig {
            host: "localhost".into(),
            port: 5432,
            user: "postgres".into(),
            password: "example".into(),
            database: "packrat".into(),
            pool_procs: 1,
        };
        assert_eq!(config, expected);
    }

    #[test]
    fn can_override_params() {
        let config = DatabaseConfig::new()
            .host("fred")
            .port(1234)
            .user("mongo")
            .password("notgonnasell")
            .database("postgres12")
            .pool_procs(5);

        let expected = DatabaseConfig {
            host: "fred".into(),
            port: 1234,
            user: "mongo".into(),
            password: "notgonnasell".into(),
            database: "postgres12".into(),
            pool_procs: 5,
        };
        assert_eq!(config, expected);
    }

    #[test]
    fn opt_none_leaves_default() {
        let none: Option<&str> = None;
        let none_num: Option<u16> = None;
        let config = DatabaseConfig::new()
            .host_opt(none)
            .port_opt(none_num)
            .user_opt(none)
            .password_opt(none)
            .database_opt(none)
            .pool_procs_opt(none_num);
        let expected = DatabaseConfig::default().pool_procs(num_cpus::get() as u16);
        assert_eq!(config, expected);
    }

    #[test]
    fn can_generate_tokio_postgres_config() {
        let config = DatabaseConfig::new()
            .host("fred")
            .port(1234)
            .user("mongo")
            .password("notgonnasell")
            .database("postgres12")
            .pool_procs(5);
        let pgconfig = config.to_postgres_config();
        let mut expected = tokio_postgres::Config::new();
        expected.host("fred");
        expected.port(1234);
        expected.user("mongo");
        expected.password("notgonnasell");
        expected.dbname("postgres12");
        assert_eq!(pgconfig, expected);
    }
}
