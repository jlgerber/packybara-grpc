use std::borrow::Borrow;
use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
//use std::net::{Ipv4Addr, Ipv6Addr};
use std::vec;
pub use url::Host as UrlHost;
use url::Url;
pub use url::{form_urlencoded, Origin, ParseError, ParseOptions, PathSegmentsMut, UrlQuery};

pub struct GrpcUrl(Url);

impl ToSocketAddrs for GrpcUrl {
    type Iter = vec::IntoIter<SocketAddr>;
    /// Convert GrpcUrl to socket addr
    fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
        let part = format!(
            "{}:{}",
            self.host_str().unwrap_or("localhost"),
            self.port().unwrap_or(80)
        );
        part.to_socket_addrs()
    }
}

impl GrpcUrl {
    /// extract socket address from GrpcUrl
    pub fn to_socket_addr(&self) -> io::Result<SocketAddr> {
        let mut result = self.to_socket_addrs()?;
        Ok(result.next().unwrap())
    }
    //// Parse an absolute URL from a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://example.net")?;
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If the function can not parse an absolute URL from the given string,
    /// a `ParseError` variant will be returned.
    #[inline]
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let url = Url::parse(input)?;
        Ok(GrpcUrl(url))
    }

    /// Parse an absolute URL from a string and add params to its query string.
    ///
    /// Existing params are not removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse_with_params("https://example.net?dont=clobberme",
    ///                                  &[("lang", "rust"), ("browser", "servo")])?;
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If the function can not parse an absolute URL from the given string,
    /// a `ParseError` variant will be returned.
    #[inline]
    pub fn parse_with_params<I, K, V>(input: &str, iter: I) -> Result<GrpcUrl, ParseError>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = Url::parse_with_params(input, iter)?;
        Ok(GrpcUrl(url))
    }

    /// Parse a string as an URL, with this URL as the base URL.
    ///
    /// Note: a trailing slash is significant.
    /// Without it, the last path component is considered to be a “file” name
    /// to be removed to get at the “directory” that is used as the base:
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let base = GrpcUrl::parse("https://example.net/a/b.html")?;
    /// let url = base.join("c.png")?;
    /// assert_eq!(url.as_str(), "https://example.net/a/c.png");  // Not /a/b.html/c.png
    ///
    /// let base = GrpcUrl::parse("https://example.net/a/b/")?;
    /// let url = base.join("c.png")?;
    /// assert_eq!(url.as_str(), "https://example.net/a/b/c.png");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If the function can not parse an URL from the given string
    /// with this URL as the base URL, a [`ParseError`] variant will be returned.
    ///
    /// `ParseError`: enum.ParseError.html
    #[inline]
    pub fn join(&self, input: &str) -> Result<GrpcUrl, ParseError> {
        let url = self.0.join(input)?;
        Ok(GrpcUrl(url))
    }

    /// Return a default `ParseOptions` that can fully configure the URL parser.
    ///
    /// # Examples
    ///
    /// Get default `ParseOptions`, then change base url
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let options = GrpcUrl::options();
    /// let api = GrpcUrl::parse("https://api.example.com")?;
    /// let base_url = options.base_url(Some(&api));
    /// let version_url = base_url.parse("version.json")?;
    /// assert_eq!(version_url.as_str(), "https://api.example.com/version.json");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn options<'a>() -> ParseOptions<'a> {
        Url::options()
    }

    /// Return the serialization of this URL.
    ///
    /// This is fast since that serialization is already stored in the `Url` struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url_str = "https://example.net/";
    /// let url = GrpcUrl::parse(url_str)?;
    /// assert_eq!(url.as_str(), url_str);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Return the serialization of this URL.
    ///
    /// This consumes the `Url` and takes ownership of the `String` stored in it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url_str = "https://example.net/";
    /// let url = GrpcUrl::parse(url_str)?;
    /// assert_eq!(url.into_string(), url_str);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    /// Return the origin of this URL (<https://url.spec.whatwg.org/#origin>)
    ///
    /// Note: this returns an opaque origin for `file:` URLs, which causes
    /// `url.origin() != url.origin()`.
    ///
    /// # Examples
    ///
    /// URL with `ftp` scheme:
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl,GrpcHost, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("ftp://example.com/foo")?;
    /// assert_eq!(url.origin(),
    ///            Origin::Tuple("ftp".into(),
    ///                          Host::Domain("example.com".into()),
    ///                          21));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// URL with `blob` scheme:
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, GrpcHost, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("blob:https://example.com/foo")?;
    /// assert_eq!(url.origin(),
    ///            Origin::Tuple("https".into(),
    ///                          Host::Domain("example.com".into()),
    ///                          443));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// URL with `file` scheme:
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError, GrpcHost, Origin};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("file:///tmp/foo")?;
    /// assert!(!url.origin().is_tuple());
    ///
    /// let other_url = GrpcUrl::parse("file:///tmp/foo")?;
    /// assert!(url.origin() != other_url.origin());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// URL with other scheme:
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("foo:bar")?;
    /// assert!(!url.origin().is_tuple());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn origin(&self) -> Origin {
        self.0.origin()
    }

    /// Return the scheme of this URL, lower-cased, as an ASCII string without the ':' delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("file:///tmp/foo")?;
    /// assert_eq!(url.scheme(), "file");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn scheme(&self) -> &str {
        self.0.scheme()
    }

    /// Return whether the URL has an 'authority',
    /// which can contain a username, password, host, and port number.
    ///
    /// URLs that do *not* are either path-only like `unix:/run/foo.socket`
    /// or cannot-be-a-base like `data:text/plain,Stuff`.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("ftp://rms@example.com")?;
    /// assert!(url.has_authority());
    ///
    /// let url = GrpcUrl::parse("unix:/run/foo.socket")?;
    /// assert!(!url.has_authority());
    ///
    /// let url = GrpcUrl::parse("data:text/plain,Stuff")?;
    /// assert!(!url.has_authority());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn has_authority(&self) -> bool {
        self.0.has_authority()
    }

    /// Return whether this URL is a cannot-be-a-base URL,
    /// meaning that parsing a relative URL string with this URL as the base will return an error.
    ///
    /// This is the case if the scheme and `:` delimiter are not followed by a `/` slash,
    /// as is typically the case of `data:` and `mailto:` URLs.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("ftp://rms@example.com")?;
    /// assert!(!url.cannot_be_a_base());
    ///
    /// let url = GrpcUrl::parse("unix:/run/foo.socket")?;
    /// assert!(!url.cannot_be_a_base());
    ///
    /// let url = GrpcUrl::parse("data:text/plain,Stuff")?;
    /// assert!(url.cannot_be_a_base());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn cannot_be_a_base(&self) -> bool {
        self.0.cannot_be_a_base()
    }

    /// Return the username for this URL (typically the empty string)
    /// as a percent-encoded ASCII string.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("ftp://rms@example.com")?;
    /// assert_eq!(url.username(), "rms");
    ///
    /// let url = GrpcUrl::parse("ftp://:secret123@example.com")?;
    /// assert_eq!(url.username(), "");
    ///
    /// let url = GrpcUrl::parse("https://example.com")?;
    /// assert_eq!(url.username(), "");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn username(&self) -> &str {
        self.0.username()
    }

    /// Return the password for this URL, if any, as a percent-encoded ASCII string.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("ftp://rms:secret123@example.com")?;
    /// assert_eq!(url.password(), Some("secret123"));
    ///
    /// let url = GrpcUrl::parse("ftp://:secret123@example.com")?;
    /// assert_eq!(url.password(), Some("secret123"));
    ///
    /// let url = GrpcUrl::parse("ftp://rms@example.com")?;
    /// assert_eq!(url.password(), None);
    ///
    /// let url = GrpcUrl::parse("https://example.com")?;
    /// assert_eq!(url.password(), None);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn password(&self) -> Option<&str> {
        self.0.password()
    }

    /// Equivalent to `url.host().is_some()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("ftp://rms@example.com")?;
    /// assert!(url.has_host());
    ///
    /// let url = GrpcUrl::parse("unix:/run/foo.socket")?;
    /// assert!(!url.has_host());
    ///
    /// let url = GrpcUrl::parse("data:text/plain,Stuff")?;
    /// assert!(!url.has_host());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn has_host(&self) -> bool {
        self.0.has_host()
    }

    /// Return the string representation of the host (domain or IP address) for this URL, if any.
    ///
    /// Non-ASCII domains are punycode-encoded per IDNA.
    /// IPv6 addresses are given between `[` and `]` brackets.
    ///
    /// Cannot-be-a-base URLs (typical of `data:` and `mailto:`) and some `file:` URLs
    /// don’t have a host.
    ///
    /// See also the `host` method.
    ///
    /// # Examples
    ///
    /// ```
    ///use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://127.0.0.1/index.html")?;
    /// assert_eq!(url.host_str(), Some("127.0.0.1"));
    ///
    /// let url = GrpcUrl::parse("ftp://rms@example.com")?;
    /// assert_eq!(url.host_str(), Some("example.com"));
    ///
    /// let url = GrpcUrl::parse("unix:/run/foo.socket")?;
    /// assert_eq!(url.host_str(), None);
    ///
    /// let url = GrpcUrl::parse("data:text/plain,Stuff")?;
    /// assert_eq!(url.host_str(), None);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn host_str(&self) -> Option<&str> {
        self.0.host_str()
    }

    /// Return the parsed representation of the host for this URL.
    /// Non-ASCII domain labels are punycode-encoded per IDNA.
    ///
    /// Cannot-be-a-base URLs (typical of `data:` and `mailto:`) and some `file:` URLs
    /// don’t have a host.
    ///
    /// See also the `host_str` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://127.0.0.1/index.html")?;
    /// assert!(url.host().is_some());
    ///
    /// let url = GrpcUrl::parse("ftp://rms@example.com")?;
    /// assert!(url.host().is_some());
    ///
    /// let url = GrpcUrl::parse("unix:/run/foo.socket")?;
    /// assert!(url.host().is_none());
    ///
    /// let url = GrpcUrl::parse("data:text/plain,Stuff")?;
    /// assert!(url.host().is_none());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn host(&self) -> Option<UrlHost<&str>> {
        self.0.host()
    }

    /// If this URL has a host and it is a domain name (not an IP address), return it.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://127.0.0.1/")?;
    /// assert_eq!(url.domain(), None);
    ///
    /// let url = GrpcUrl::parse("mailto:rms@example.net")?;
    /// assert_eq!(url.domain(), None);
    ///
    /// let url = GrpcUrl::parse("https://example.com/")?;
    /// assert_eq!(url.domain(), Some("example.com"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    /// Return the port number for this URL, if any.
    ///
    /// Note that default port numbers are never reflected by the serialization,
    /// use the `port_or_known_default()` method if you want a default port number returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://example.com")?;
    /// assert_eq!(url.port(), None);
    ///
    /// let url = GrpcUrl::parse("https://example.com:443/")?;
    /// assert_eq!(url.port(), None);
    ///
    /// let url = GrpcUrl::parse("ssh://example.com:22")?;
    /// assert_eq!(url.port(), Some(22));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn port(&self) -> Option<u16> {
        self.0.port()
    }

    /// Return the port number for this URL, or the default port number if it is known.
    ///
    /// This method only knows the default port number
    /// of the `http`, `https`, `ws`, `wss`, `ftp`, and `gopher` schemes.
    ///
    /// For URLs in these schemes, this method always returns `Some(_)`.
    /// For other schemes, it is the same as `Url::port()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("foo://example.com")?;
    /// assert_eq!(url.port_or_known_default(), None);
    ///
    /// let url = GrpcUrl::parse("foo://example.com:1456")?;
    /// assert_eq!(url.port_or_known_default(), Some(1456));
    ///
    /// let url = GrpcUrl::parse("https://example.com")?;
    /// assert_eq!(url.port_or_known_default(), Some(443));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn port_or_known_default(&self) -> Option<u16> {
        self.0.port_or_known_default()
    }

    /// Resolve a URL’s host and port number to `SocketAddr`.
    ///
    /// If the URL has the default port number of a scheme that is unknown to this library,
    /// `default_port_number` provides an opportunity to provide the actual port number.
    /// In non-example code this should be implemented either simply as `|| None`,
    /// or by matching on the URL’s `.scheme()`.
    ///
    /// If the host is a domain, it is resolved using the standard library’s DNS support.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// let url = GrpcUrl::parse("https://example.net/").unwrap();
    /// let addrs = url.socket_addrs(|| None).unwrap();
    /// std::net::TcpStream::connect(&*addrs)
    /// # ;
    /// ```
    ///
    /// ```
    /// /// With application-specific known default port numbers
    /// fn socket_addrs(url: GrpcUrl) -> std::io::Result<Vec<std::net::SocketAddr>> {
    ///     url.socket_addrs(|| match url.scheme() {
    ///         "socks5" | "socks5h" => Some(1080),
    ///         _ => None,
    ///     })
    /// }
    /// ```
    pub fn socket_addrs(
        &self,
        default_port_number: impl Fn() -> Option<u16>,
    ) -> std::io::Result<Vec<SocketAddr>> {
        self.0.socket_addrs(default_port_number)
    }

    /// Return the path for this URL, as a percent-encoded ASCII string.
    /// For cannot-be-a-base URLs, this is an arbitrary string that doesn’t start with '/'.
    /// For other URLs, this starts with a '/' slash
    /// and continues with slash-separated path segments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://example.com/api/versions?page=2")?;
    /// assert_eq!(url.path(), "/api/versions");
    ///
    /// let url = GrpcUrl::parse("https://example.com")?;
    /// assert_eq!(url.path(), "/");
    ///
    /// let url = GrpcUrl::parse("https://example.com/countries/việt nam")?;
    /// assert_eq!(url.path(), "/countries/vi%E1%BB%87t%20nam");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn path(&self) -> &str {
        self.0.path()
    }

    /// Unless this URL is cannot-be-a-base,
    /// return an iterator of '/' slash-separated path segments,
    /// each as a percent-encoded ASCII string.
    ///
    /// Return `None` for cannot-be-a-base URLs.
    ///
    /// When `Some` is returned, the iterator always contains at least one string
    /// (which may be empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), Box<Error>> {
    /// let url = GrpcUrl::parse("https://example.com/foo/bar")?;
    /// let mut path_segments = url.path_segments().ok_or_else(|| "cannot be base")?;
    /// assert_eq!(path_segments.next(), Some("foo"));
    /// assert_eq!(path_segments.next(), Some("bar"));
    /// assert_eq!(path_segments.next(), None);
    ///
    /// let url = GrpcUrl::parse("https://example.com")?;
    /// let mut path_segments = url.path_segments().ok_or_else(|| "cannot be base")?;
    /// assert_eq!(path_segments.next(), Some(""));
    /// assert_eq!(path_segments.next(), None);
    ///
    /// let url = GrpcUrl::parse("data:text/plain,HelloWorld")?;
    /// assert!(url.path_segments().is_none());
    ///
    /// let url = GrpcUrl::parse("https://example.com/countries/việt nam")?;
    /// let mut path_segments = url.path_segments().ok_or_else(|| "cannot be base")?;
    /// assert_eq!(path_segments.next(), Some("countries"));
    /// assert_eq!(path_segments.next(), Some("vi%E1%BB%87t%20nam"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn path_segments(&self) -> Option<std::str::Split<char>> {
        self.0.path_segments()
    }

    /// Return this URL’s query string, if any, as a percent-encoded ASCII string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://example.com/products?page=2")?;
    /// let query = url.query();
    /// assert_eq!(query, Some("page=2"));
    ///
    /// let url = Url::parse("https://example.com/products")?;
    /// let query = url.query();
    /// assert!(query.is_none());
    ///
    /// let url = GrpcUrl::parse("https://example.com/?country=español")?;
    /// let query = url.query();
    /// assert_eq!(query, Some("country=espa%C3%B1ol"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn query(&self) -> Option<&str> {
        self.0.query()
    }

    /// Parse the URL’s query string, if any, as `application/x-www-form-urlencoded`
    /// and return an iterator of (key, value) pairs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::borrow::Cow;
    ///
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://example.com/products?page=2&sort=desc")?;
    /// let mut pairs = url.query_pairs();
    ///
    /// assert_eq!(pairs.count(), 2);
    ///
    /// assert_eq!(pairs.next(), Some((Cow::Borrowed("page"), Cow::Borrowed("2"))));
    /// assert_eq!(pairs.next(), Some((Cow::Borrowed("sort"), Cow::Borrowed("desc"))));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    ///

    #[inline]
    pub fn query_pairs(&self) -> form_urlencoded::Parse {
        self.0.query_pairs()
    }

    /// Return this URL’s fragment identifier, if any.
    ///
    /// A fragment is the part of the URL after the `#` symbol.
    /// The fragment is optional and, if present, contains a fragment identifier
    /// that identifies a secondary resource, such as a section heading
    /// of a document.
    ///
    /// In HTML, the fragment identifier is usually the id attribute of a an element
    /// that is scrolled to on load. Browsers typically will not send the fragment portion
    /// of a URL to the server.
    ///
    /// **Note:** the parser did *not* percent-encode this component,
    /// but the input may have been percent-encoded already.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let url = GrpcUrl::parse("https://example.com/data.csv#row=4")?;
    ///
    /// assert_eq!(url.fragment(), Some("row=4"));
    ///
    /// let url = GrpcUrl::parse("https://example.com/data.csv#cell=4,1-6,2")?;
    ///
    /// assert_eq!(url.fragment(), Some("cell=4,1-6,2"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn fragment(&self) -> Option<&str> {
        self.0.fragment()
    }

    /// Change this URL’s fragment identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("https://example.com/data.csv")?;
    /// assert_eq!(url.as_str(), "https://example.com/data.csv");

    /// url.set_fragment(Some("cell=4,1-6,2"));
    /// assert_eq!(url.as_str(), "https://example.com/data.csv#cell=4,1-6,2");
    /// assert_eq!(url.fragment(), Some("cell=4,1-6,2"));
    ///
    /// url.set_fragment(None);
    /// assert_eq!(url.as_str(), "https://example.com/data.csv");
    /// assert!(url.fragment().is_none());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        self.0.set_fragment(fragment)
    }

    /// Change this URL’s query string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("https://example.com/products")?;
    /// assert_eq!(url.as_str(), "https://example.com/products");
    ///
    /// url.set_query(Some("page=2"));
    /// assert_eq!(url.as_str(), "https://example.com/products?page=2");
    /// assert_eq!(url.query(), Some("page=2"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn set_query(&mut self, query: Option<&str>) {
        self.0.set_query(query)
    }

    /// Manipulate this URL’s query string, viewed as a sequence of name/value pairs
    /// in `application/x-www-form-urlencoded` syntax.
    ///
    /// The return value has a method-chaining API:
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("https://example.net?lang=fr#nav")?;
    /// assert_eq!(url.query(), Some("lang=fr"));
    ///
    /// url.query_pairs_mut().append_pair("foo", "bar");
    /// assert_eq!(url.query(), Some("lang=fr&foo=bar"));
    /// assert_eq!(url.as_str(), "https://example.net/?lang=fr&foo=bar#nav");
    ///
    /// url.query_pairs_mut()
    ///     .clear()
    ///     .append_pair("foo", "bar & baz")
    ///     .append_pair("saisons", "\u{00C9}t\u{00E9}+hiver");
    /// assert_eq!(url.query(), Some("foo=bar+%26+baz&saisons=%C3%89t%C3%A9%2Bhiver"));
    /// assert_eq!(url.as_str(),
    ///            "https://example.net/?foo=bar+%26+baz&saisons=%C3%89t%C3%A9%2Bhiver#nav");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Note: `url.query_pairs_mut().clear();` is equivalent to `url.set_query(Some(""))`,
    /// not `url.set_query(None)`.
    ///
    /// The state of `Url` is unspecified if this return value is leaked without being dropped.
    pub fn query_pairs_mut(&mut self) -> form_urlencoded::Serializer<UrlQuery> {
        self.0.query_pairs_mut()
    }

    /// Change this URL’s path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("https://example.com")?;
    /// url.set_path("api/comments");
    /// assert_eq!(url.as_str(), "https://example.com/api/comments");
    /// assert_eq!(url.path(), "/api/comments");
    ///
    /// let mut url = GrpcUrl::parse("https://example.com/api")?;
    /// url.set_path("data/report.csv");
    /// assert_eq!(url.as_str(), "https://example.com/data/report.csv");
    /// assert_eq!(url.path(), "/data/report.csv");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn set_path(&mut self, path: &str) {
        self.0.set_path(path)
    }

    /// Return an object with methods to manipulate this URL’s path segments.
    ///
    /// Return `Err(())` if this URL is cannot-be-a-base.
    pub fn path_segments_mut(&mut self) -> Result<PathSegmentsMut, ()> {
        self.0.path_segments_mut()
    }

    /// Change this URL’s port number.
    ///
    /// Note that default port numbers are not reflected in the serialization.
    ///
    /// If this URL is cannot-be-a-base, does not have a host, or has the `file` scheme;
    /// do nothing and return `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), Box<Error>> {
    /// let mut url = GrpcUrl::parse("ssh://example.net:2048/")?;
    ///
    /// url.set_port(Some(4096)).map_err(|_| "cannot be base")?;
    /// assert_eq!(url.as_str(), "ssh://example.net:4096/");
    ///
    /// url.set_port(None).map_err(|_| "cannot be base")?;
    /// assert_eq!(url.as_str(), "ssh://example.net/");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Known default port numbers are not reflected:
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), Box<Error>> {
    /// let mut url = GrpcUrl::parse("https://example.org/")?;
    ///
    /// url.set_port(Some(443)).map_err(|_| "cannot be base")?;
    /// assert!(url.port().is_none());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Cannot set port for cannot-be-a-base URLs:
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("mailto:rms@example.net")?;
    ///
    /// let result = url.set_port(Some(80));
    /// assert!(result.is_err());
    ///
    /// let result = url.set_port(None);
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn set_port(&mut self, port: Option<u16>) -> Result<(), ()> {
        self.0.set_port(port)
    }

    /// Change this URL’s host.
    ///
    /// Removing the host (calling this with `None`)
    /// will also remove any username, password, and port number.
    ///
    /// # Examples
    ///
    /// Change host:
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("https://example.net")?;
    /// let result = url.set_host(Some("rust-lang.org"));
    /// assert!(result.is_ok());
    /// assert_eq!(url.as_str(), "https://rust-lang.org/");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Remove host:
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("foo://example.net")?;
    /// let result = url.set_host(None);
    /// assert!(result.is_ok());
    /// assert_eq!(url.as_str(), "foo:/");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Cannot remove host for 'special' schemes (e.g. `http`):
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("https://example.net")?;
    /// let result = url.set_host(None);
    /// assert!(result.is_err());
    /// assert_eq!(url.as_str(), "https://example.net/");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Cannot change or remove host for cannot-be-a-base URLs:
    ///
    /// ```
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("mailto:rms@example.net")?;
    ///
    /// let result = url.set_host(Some("rust-lang.org"));
    /// assert!(result.is_err());
    /// assert_eq!(url.as_str(), "mailto:rms@example.net");
    ///
    /// let result = url.set_host(None);
    /// assert!(result.is_err());
    /// assert_eq!(url.as_str(), "mailto:rms@example.net");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// If this URL is cannot-be-a-base or there is an error parsing the given `host`,
    /// a `ParseError` variant will be returned.
    pub fn set_host(&mut self, host: Option<&str>) -> Result<(), ParseError> {
        self.0.set_host(host)
    }

    /// Change this URL’s host to the given IP address.
    ///
    /// If this URL is cannot-be-a-base, do nothing and return `Err`.
    ///
    /// Compared to `Url::set_host`, this skips the host parser.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use url::{Url, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("http://example.com")?;
    /// url.set_ip_host("127.0.0.1".parse().unwrap());
    /// assert_eq!(url.host_str(), Some("127.0.0.1"));
    /// assert_eq!(url.as_str(), "http://127.0.0.1/");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Cannot change URL's from mailto(cannot-be-base) to ip:
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("mailto:rms@example.com")?;
    /// let result = url.set_ip_host("127.0.0.1".parse().unwrap());
    ///
    /// assert_eq!(url.as_str(), "mailto:rms@example.com");
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    pub fn set_ip_host(&mut self, address: std::net::IpAddr) -> Result<(), ()> {
        self.0.set_ip_host(address)
    }

    /// Change this URL’s password.
    ///
    /// If this URL is cannot-be-a-base or does not have a host, do nothing and return `Err`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use packybara_grpc::url_builder::{GrpcUrl, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = GrpcUrl::parse("mailto:rmz@example.com")?;
    /// let result = url.set_password(Some("secret_password"));
    /// assert!(result.is_err());
    ///
    /// let mut url = GrpcUrl::parse("ftp://user1:secret1@example.com")?;
    /// let result = url.set_password(Some("secret_password"));
    /// assert_eq!(url.password(), Some("secret_password"));
    ///
    /// let mut url = GrpcUrl::parse("ftp://user2:@example.com")?;
    /// let result = url.set_password(Some("secret2"));
    /// assert!(result.is_ok());
    /// assert_eq!(url.password(), Some("secret2"));
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn set_password(&mut self, password: Option<&str>) -> Result<(), ()> {
        self.0.set_password(password)
    }

    /// Change this URL’s username.
    ///
    /// If this URL is cannot-be-a-base or does not have a host, do nothing and return `Err`.
    /// # Examples
    ///
    /// Cannot setup username from mailto(cannot-be-base)
    ///
    /// ```rust
    /// use url::{Url, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("mailto:rmz@example.com")?;
    /// let result = url.set_username("user1");
    /// assert_eq!(url.as_str(), "mailto:rmz@example.com");
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Setup username to user1
    ///
    /// ```rust
    /// use url::{Url, ParseError};
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("ftp://:secre1@example.com/")?;
    /// let result = url.set_username("user1");
    /// assert!(result.is_ok());
    /// assert_eq!(url.username(), "user1");
    /// assert_eq!(url.as_str(), "ftp://user1:secre1@example.com/");
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn set_username(&mut self, username: &str) -> Result<(), ()> {
        self.0.set_username(username)
    }

    /// Change this URL’s scheme.
    ///
    /// Do nothing and return `Err` if:
    ///
    /// * The new scheme is not in `[a-zA-Z][a-zA-Z0-9+.-]+`
    /// * This URL is cannot-be-a-base and the new scheme is one of
    ///   `http`, `https`, `ws`, `wss`, `ftp`, or `gopher`
    ///
    /// # Examples
    ///
    /// Change the URL’s scheme from `https` to `foo`:
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("https://example.net")?;
    /// let result = url.set_scheme("http");
    /// assert_eq!(url.as_str(), "http://example.net/");
    /// assert!(result.is_ok());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    /// Change the URL’s scheme from `foo` to `bar`:
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("foo://example.net")?;
    /// let result = url.set_scheme("bar");
    /// assert_eq!(url.as_str(), "bar://example.net");
    /// assert!(result.is_ok());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Cannot change URL’s scheme from `https` to `foõ`:
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("https://example.net")?;
    /// let result = url.set_scheme("foõ");
    /// assert_eq!(url.as_str(), "https://example.net/");
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    ///
    /// Cannot change URL’s scheme from `mailto` (cannot-be-a-base) to `https`:
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("mailto:rms@example.net")?;
    /// let result = url.set_scheme("https");
    /// assert_eq!(url.as_str(), "mailto:rms@example.net");
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    /// Cannot change the URL’s scheme from `foo` to `https`:
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("foo://example.net")?;
    /// let result = url.set_scheme("https");
    /// assert_eq!(url.as_str(), "foo://example.net");
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    /// Cannot change the URL’s scheme from `http` to `foo`:
    ///
    /// ```
    /// use url::Url;
    /// # use url::ParseError;
    ///
    /// # fn run() -> Result<(), ParseError> {
    /// let mut url = Url::parse("http://example.net")?;
    /// let result = url.set_scheme("foo");
    /// assert_eq!(url.as_str(), "http://example.net/");
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    pub fn set_scheme(&mut self, scheme: &str) -> Result<(), ()> {
        self.0.set_scheme(scheme)
    }

    /// Convert a file name as `std::path::Path` into an URL in the `file` scheme.
    ///
    /// This returns `Err` if the given path is not absolute or,
    /// on Windows, if the prefix is not a disk prefix (e.g. `C:`) or a UNC prefix (`\\`).
    ///
    /// # Examples
    ///
    /// On Unix-like platforms:
    ///
    /// ```
    /// # if cfg!(unix) {
    /// use url::Url;
    ///
    /// # fn run() -> Result<(), ()> {
    /// let url = Url::from_file_path("/tmp/foo.txt")?;
    /// assert_eq!(url.as_str(), "file:///tmp/foo.txt");
    ///
    /// let url = Url::from_file_path("../foo.txt");
    /// assert!(url.is_err());
    ///
    /// let url = Url::from_file_path("https://google.com/");
    /// assert!(url.is_err());
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// # }
    /// ```
    #[cfg(any(unix, windows, target_os = "redox"))]
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<GrpcUrl, ()> {
        let url = Url::from_file_path(path)?;
        Ok(GrpcUrl(url))
    }

    /// Convert a directory name as `std::path::Path` into an URL in the `file` scheme.
    ///
    /// This returns `Err` if the given path is not absolute or,
    /// on Windows, if the prefix is not a disk prefix (e.g. `C:`) or a UNC prefix (`\\`).
    ///
    /// Compared to `from_file_path`, this ensure that URL’s the path has a trailing slash
    /// so that the entire path is considered when using this URL as a base URL.
    ///
    /// For example:
    ///
    /// * `"index.html"` parsed with `Url::from_directory_path(Path::new("/var/www"))`
    ///   as the base URL is `file:///var/www/index.html`
    /// * `"index.html"` parsed with `Url::from_file_path(Path::new("/var/www"))`
    ///   as the base URL is `file:///var/index.html`, which might not be what was intended.
    ///
    /// Note that `std::path` does not consider trailing slashes significant
    /// and usually does not include them (e.g. in `Path::parent()`).
    #[cfg(any(unix, windows, target_os = "redox"))]
    pub fn from_directory_path<P: AsRef<std::path::Path>>(path: P) -> Result<GrpcUrl, ()> {
        let url = Url::from_directory_path(path)?;
        Ok(GrpcUrl(url))
    }

    /// Serialize with Serde using the internal representation of the `Url` struct.
    ///
    /// The corresponding `deserialize_internal` method sacrifices some invariant-checking
    /// for speed, compared to the `Deserialize` trait impl.
    ///
    /// This method is only available if the `serde` Cargo feature is enabled.
    #[cfg(feature = "serde")]
    #[deny(unused)]
    pub fn serialize_internal<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize_internal(serializer)
    }

    /// Serialize with Serde using the internal representation of the `Url` struct.
    ///
    /// The corresponding `deserialize_internal` method sacrifices some invariant-checking
    /// for speed, compared to the `Deserialize` trait impl.
    ///
    /// This method is only available if the `serde` Cargo feature is enabled.
    #[cfg(feature = "serde")]
    #[deny(unused)]
    pub fn deserialize_internal<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let url = Url::deserialize_internal(deserizer)?;
        Ok(GrpcUrl(url))
    }
}

/// Parse a string as an URL, without a base URL or encoding override.
impl std::str::FromStr for GrpcUrl {
    type Err = ParseError;

    #[inline]
    fn from_str(input: &str) -> Result<GrpcUrl, ParseError> {
        GrpcUrl::parse(input)
    }
}

/// Display the serialization of this URL.
impl std::fmt::Display for GrpcUrl {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}

/// Debug the serialization of this URL.
impl std::fmt::Debug for GrpcUrl {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), formatter)
    }
}

/// URLs compare like their serialization.
impl Eq for GrpcUrl {}

/// URLs compare like their serialization.
impl PartialEq for GrpcUrl {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

/// URLs compare like their serialization.
impl Ord for GrpcUrl {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

/// URLs compare like their serialization.
impl PartialOrd for GrpcUrl {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

/// URLs hash like their serialization.
impl std::hash::Hash for GrpcUrl {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        std::hash::Hash::hash(self.as_str(), state)
    }
}

/// Return the serialization of this URL.
impl AsRef<str> for GrpcUrl {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn can_convert_to_socket_addr() {
        let url = GrpcUrl::from_str("http://localhost");
        assert!(url.is_ok());
        let url = url.unwrap();
        let result = url.to_socket_addr().unwrap();
        //let rstr = format!("{:?}", result);
        //assert_eq!(rstr.as_str(), "http://127.0.0.1:8080/");
    }
}
