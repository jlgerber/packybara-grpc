use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use url::Url;

/// The scheme may either be http or https
#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
    Http,
    Https,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http => write!(f, "http"),
            Self::Https => write!(f, "https"),
        }
    }
}

/// Models a Fully Qualified Domain Name, consisting of a host, a domain, and a top level domain.
#[derive(Debug, PartialEq, Eq)]
pub struct Fqdn {
    name: String,
    host_sz: u8,
    domain_sz: u8,
    tld_sz: u8,
}

impl fmt::Display for Fqdn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Fqdn {
    /// New up an Fqdn instance from the a host, domain, and top level domain
    ///
    /// # Arguments
    ///
    /// * `host` - The host name (typically wwww)
    /// * `domain` - The domain name ( eg github)
    /// * `tld` - The `top level domain` (eg com)
    ///
    /// # Returns
    ///
    /// * Fqdn instance
    pub fn new(host: &str, domain: &str, tld: &str) -> Self {
        Self {
            name: format!("{}.{}.{}", host, domain, tld),
            host_sz: host.len() as u8,
            domain_sz: domain.len() as u8,
            tld_sz: tld.len() as u8,
        }
    }

    /// Construction function which builds an Fqdn from a str representation.
    /// The function is fallible, and will error if, for example, the wrong
    /// number of parts are supplied (ie greater than 3 or less than 2)
    // TODO:: replace string with proper error
    pub fn from_str(fqdn: &str) -> Result<Fqdn, String> {
        let pieces = fqdn.split(".").into_iter().collect::<Vec<_>>();
        if pieces.len() > 3 || pieces.len() < 2 {
            Err(format!("Unable to construct Fqdn from '{}'", fqdn))
        } else if pieces.len() == 2 {
            Ok(Fqdn::new("www", pieces[0], pieces[1]))
        } else {
            Ok(Fqdn::new(pieces[0], pieces[1], pieces[2]))
        }
    }

    /// Retrieve the host name from the Fqdn as a &str
    pub fn host(&self) -> &str {
        &self.name[..self.host_sz as usize]
    }

    /// Retrieve the domain name from the Fqdn as a &str
    pub fn domain(&self) -> &str {
        &self.name[(self.host_sz + 1) as usize..(self.host_sz + self.domain_sz + 1) as usize]
    }

    /// Retrieve the top level domain from the Fqdn as a &str
    pub fn tld(&self) -> &str {
        &self.name[(self.host_sz + self.domain_sz + 2) as usize
            ..(self.host_sz + self.domain_sz + self.tld_sz + 2) as usize]
    }

    /// Returns a &str representation of the Fqdn
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

/// Different expressions of a host, including raw ip addresses, fully qualified domain name, and localhost
#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    Fqdn(Fqdn),
    Localhost,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V4(v4) => v4.fmt(f),
            Self::V6(v6) => v6.fmt(f),
            Self::Fqdn(fqdn) => fqdn.fmt(f),
            Self::Localhost => write!(f, "localhost"),
        }
    }
}

/// Implements construction of Url via the Builder pattern
#[derive(Debug, PartialEq, Eq)]
pub struct UrlBuilder {
    scheme: Option<Scheme>,
    host: Option<Host>,
    port: Option<u16>,
    route: Option<String>,
}

impl Default for UrlBuilder {
    fn default() -> Self {
        Self {
            scheme: Some(Scheme::Http),
            host: Some(Host::Localhost),
            port: Some(80),
            route: None,
        }
    }
}
impl UrlBuilder {
    /// New up a UrlBuilder instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the scheme for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern
    ///
    /// # Arguments
    ///
    /// * `scheme` - The scheme enum representing http or https
    ///
    /// # Returns
    ///
    /// * Self
    pub fn scheme(mut self, scheme: Scheme) -> Self {
        self.scheme = Some(scheme);
        self
    }

    /// Set the Optional scheme for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern
    ///
    /// # Arguments
    ///
    /// * `scheme` - The scheme enum representing http or https, wrapped in an Option
    ///
    /// # Returns
    ///
    /// * Self
    pub fn scheme_opt(mut self, scheme: Option<Scheme>) -> Self {
        self.scheme = scheme;
        self
    }

    /// Set the host for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern
    ///
    /// # Arguments
    ///
    /// * `host` - The host enum representing an ip address or fqdn
    ///
    /// # Returns
    ///
    /// * Self
    pub fn host(mut self, host: Host) -> Self {
        self.host = Some(host);
        self
    }

    /// Set the Optional host for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern
    ///
    /// # Arguments
    ///
    /// * `host` - The host enum representing an ip address or fqdn, wrapped in an Option
    ///
    /// # Returns
    ///
    /// * Self
    pub fn host_opt(mut self, host: Option<Host>) -> Self {
        self.host = host;
        self
    }

    /// Set the port for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern
    ///
    /// # Arguments
    ///
    /// * `port` - The port as a u16
    ///
    /// # Returns
    ///
    /// * Self
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set the Optional port for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern
    ///
    /// # Arguments
    ///
    /// * `port` - The port as a u16 wrapped in an Option
    ///
    /// # Returns
    ///
    /// * Self
    pub fn port_opt(mut self, port: Option<u16>) -> Self {
        self.port = port;
        self
    }

    /// Set the route for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern. The route follows the `scheme://host:port/`.
    /// ie `scheme://host:port/route`
    ///
    /// # Arguments
    ///
    /// * `route` - The route as a u16
    ///
    /// # Returns
    ///
    /// * Self
    pub fn route<I>(mut self, route: I) -> Self
    where
        I: Into<String>,
    {
        self.route = Some(route.into());
        self
    }

    /// Set the optional route for the UrlBuilder and return an instance of Self,
    /// per the Builder pattern. The route follows the `scheme://host:port/`.
    /// ie `scheme://host:port/route`
    ///
    /// # Arguments
    ///
    /// * `route` - The route as a u16 wrapped in an Option
    ///
    /// # Returns
    ///
    /// * Self
    pub fn route_opt(mut self, route: Option<String>) -> Self {
        self.route = route;
        self
    }

    pub fn build(self) -> Url {
        Url::parse(&format!(
            "{}://{}:{}/{}",
            self.scheme.as_ref().unwrap(), //.unwrap_or(&Scheme::Http),
            self.host.as_ref().unwrap(),   //.unwrap_or(&Host::Localhost),
            self.port.unwrap(),            //.unwrap_or(80),
            self.route.as_deref().unwrap_or("")
        ))
        .expect("unable to create url")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_fqdn_from_3item_str() {
        let fqdn = Fqdn::from_str("www.github.com");
        let expect = Fqdn::new("www", "github", "com");
        assert_eq!(fqdn, Ok(expect));
    }
    #[test]
    fn can_build_fqdn_from_2item_str() {
        let fqdn = Fqdn::from_str("github.com");
        let expect = Fqdn::new("www", "github", "com");
        assert_eq!(fqdn, Ok(expect));
    }
    #[test]
    fn cannot_build_fqdn_from_1item_str() {
        let fqdn = Fqdn::from_str("github");
        assert!(fqdn.is_err());
    }

    #[test]
    fn cannot_build_fqdn_from_4item_str() {
        let fqdn = Fqdn::from_str("www.github.com.com");
        assert!(fqdn.is_err());
    }
    #[test]
    fn can_extract_host() {
        let fqdn = Fqdn::new("wwww", "github", "com");
        let host = fqdn.host();
        assert_eq!(host, "wwww");
    }

    #[test]
    fn can_extract_domain() {
        let fqdn = Fqdn::new("wwww", "github", "com");
        let domain = fqdn.domain();
        assert_eq!(domain, "github");
    }

    #[test]
    fn can_extract_tld() {
        let fqdn = Fqdn::new("wwww", "github", "com");
        let tld = fqdn.tld();
        assert_eq!(tld, "com");
    }

    #[test]
    fn can_build_url_with_fqdn() {
        let url = UrlBuilder::new()
            .scheme(Scheme::Https)
            .host(Host::Fqdn(Fqdn::from_str("github.com").unwrap()))
            .port(8080)
            .build();
        let expect = "https://www.github.com:8080/";
        assert_eq!(url.as_str(), expect);
    }

    #[test]
    fn can_build_url_with_ipv4() {
        let url = UrlBuilder::new()
            .scheme(Scheme::Https)
            .host(Host::V4(Ipv4Addr::new(192, 168, 1, 1)))
            .port(8080)
            .build();
        let expect = "https://192.168.1.1:8080/";
        assert_eq!(url.as_str(), expect);
    }
    #[test]
    fn can_build_url_from_default() {
        let url = UrlBuilder::new().build();
        // when port is set to 80, it is implicit
        let expect = "http://localhost/";
        assert_eq!(url.as_str(), expect);
    }

    #[test]
    fn can_build_url_from_default_8080_port() {
        let url = UrlBuilder::new().port(8080).build();
        let expect = "http://localhost:8080/";
        assert_eq!(url.as_str(), expect);
    }
}
