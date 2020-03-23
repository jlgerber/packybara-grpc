use super::*;

pub mod get_pkgcoords {
    /// Encapsulate the query parameters
    pub struct Options {
        pub package: Option<String>,
        pub level: Option<String>,
        pub role: Option<String>,
        pub platform: Option<String>,
        pub site: Option<String>,
        pub search_mode: Option<String>,
        pub order_by: Option<String>,
    }

    impl Options {
        /// New up an instance of get_pkgcoords::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `level` - the optional field to level by
        /// * `role`
        /// * `platform`
        /// * `site`
        /// * `search_mode`
        /// * `order_by` - the optional direction to order by
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                package: None,
                level: None,
                role: None,
                platform: None,
                site: None,
                search_mode: None,
                order_by: None,
            }
        }

        pub fn package_opt(mut self, package: Option<String>) -> Self {
            self.package = package;
            self
        }

        pub fn level_opt(mut self, level: Option<String>) -> Self {
            self.level = level;
            self
        }
        pub fn role_opt(mut self, role: Option<String>) -> Self {
            self.role = role;
            self
        }
        pub fn platform_opt(mut self, platform: Option<String>) -> Self {
            self.platform = platform;
            self
        }
        pub fn site_opt(mut self, site: Option<String>) -> Self {
            self.site = site;
            self
        }
        pub fn search_mode_opt(mut self, search_mode: Option<String>) -> Self {
            self.search_mode = search_mode;
            self
        }
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
    }
    use super::*;
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: get_pkgcoords::Options,
    ) -> Result<Vec<FindAllPkgCoordsRow>, Box<dyn std::error::Error>> {
        let get_pkgcoords::Options {
            package,
            level,
            role,
            platform,
            site,
            search_mode,
            order_by,
        } = options;
        let request = tonic::Request::new(PkgCoordsQueryRequest {
            package,
            level,
            role,
            platform,
            site,
            search_mode,
            order_by,
        });
        let response = grpc_client.client.get_pkg_coords(request).await?;
        let PkgCoordsQueryReply { pkgcoords } = response.into_inner();

        let results = pkgcoords
            .into_iter()
            .map(|coord| {
                let PkgCoordsQueryRow {
                    id,
                    package,
                    level,
                    role,
                    platform,
                    site,
                } = coord;
                FindAllPkgCoordsRow::from_parts(
                    id as i32, &package, &level, &role, &platform, &site,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
