use crate::{setup::SNAKE_CASE_DEFAULT, Protocol};
pub use dep::Dep;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub mod de;
mod dep;
pub mod ser;

#[derive(Clone, Debug)]
pub struct Service {
    pub(super) auth: bool, // both authentication and authorization
    pub(super) database: bool,
    pub(super) deps: Vec<Dep>,
    pub(super) entries: Vec<DirEntry>,
    pub(super) graphql: bool,
    pub(super) http_client: bool,
    pub(super) mailer: bool,
    pub(super) messaging: bool,
    pub(super) monitoring: bool,
    pub(super) name: String,
    pub(super) path: PathBuf,
    pub(super) protocol: Protocol,
    pub(super) ssh: bool,
}

impl Service {
    pub fn new(name: &str, path: &str) -> Self {
        Service {
            auth: true,
            database: true,
            deps: vec![],
            entries: vec![],
            graphql: true,
            http_client: true,
            mailer: true,
            monitoring: true,
            messaging: true,
            name: name.to_string(),
            path: PathBuf::from(path),
            protocol: Protocol::Rest,
            ssh: false,
        }
    }

    pub fn default() -> Self {
        Service::new(SNAKE_CASE_DEFAULT, &format!("./{}", SNAKE_CASE_DEFAULT))
    }

    pub fn set_deps(&mut self, deps: Vec<Dep>) -> &mut Service {
        self.deps = deps;

        self
    }

    pub fn set_entries(&mut self) -> &mut Service {
        // FIXME não podemos só ignorar os erros
        // na leituras dos arquivos
        self.entries = WalkDir::new(&self.path)
            .same_file_system(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        self
    }

    pub fn set_no_auth(&mut self, no_auth: bool) -> &mut Service {
        if no_auth {
            self.auth = false;
            self.deps.retain(|d| !d.is_auth());

            return self;
        }

        self
    }

    pub fn set_no_database(&mut self, no_database: bool) -> &mut Service {
        if no_database {
            self.database = false;
            self.deps.retain(|d| !d.is_database());

            return self;
        }

        self
    }

    pub fn set_no_graphql(&mut self, no_graphql: bool) -> &mut Service {
        if no_graphql {
            self.graphql = false;
            self.deps.retain(|d| !d.is_graphql());

            return self;
        }

        self
    }

    pub fn set_no_http_client(&mut self, no_http_client: bool) -> &mut Service {
        if no_http_client {
            self.http_client = false;
            self.deps.retain(|d| !d.is_http_client());

            return self;
        }

        self
    }

    pub fn set_no_mailer(&mut self, no_mailer: bool) -> &mut Service {
        if no_mailer {
            self.mailer = false;
            self.deps.retain(|d| !d.is_mailer());

            return self;
        }

        self
    }

    pub fn set_no_messaging(&mut self, no_messaging: bool) -> &mut Service {
        if no_messaging {
            self.messaging = false;
            self.deps.retain(|d| !d.is_messaging());

            return self;
        }

        self
    }

    pub fn set_no_monitoring(&mut self, no_monitoring: bool) -> &mut Service {
        if no_monitoring {
            self.monitoring = false;
            self.deps.retain(|d| !d.is_monitoring());

            return self;
        }

        self
    }

    pub fn set_protocol(&mut self, protocol: Protocol) -> &mut Service {
        if protocol.is_grpc() {
            self.protocol = protocol;
            self.set_no_rest();

            return self;
        }

        self.protocol = protocol;
        self.set_no_grpc();

        self
    }

    pub fn set_ssh(&mut self, ssh: bool) -> &mut Service {
        // SSH is disabled by default
        self.ssh = ssh;

        self
    }

    fn set_no_grpc(&mut self) {
        // TODO remove all grpc stuff
        self.deps.retain(|d| !d.is_grpc())
    }

    fn set_no_rest(&mut self) {
        // TODO remove all phoenix stuff
        self.set_no_graphql(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const CARGO_ROOT: &str = env!("CARGO_MANIFEST_DIR");

    #[test]
    fn set_no_auth() {
        let mut default_service = Service::default();

        assert_eq!(default_service.auth, true);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_no_auth(true);

        assert_eq!(service.auth, false);
        assert!(service.deps.iter().all(|d| !d.is_auth()));
    }

    #[test]
    fn set_no_database() {
        let mut default_service = Service::default();

        assert_eq!(default_service.database, true);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_no_database(true);

        assert_eq!(service.database, false);
        assert!(service.deps.iter().all(|d| !d.is_database()));
    }

    #[test]
    fn set_no_graphql() {
        let mut default_service = Service::default();

        assert_eq!(default_service.graphql, true);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_no_graphql(true);

        assert_eq!(service.graphql, false);
        assert!(service.deps.iter().all(|d| !d.is_graphql()));
    }

    #[test]
    fn set_no_http_client() {
        let mut default_service = Service::default();

        assert_eq!(default_service.http_client, true);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_no_http_client(true);

        assert_eq!(service.http_client, false);
        assert!(service.deps.iter().all(|d| !d.is_http_client()));
    }

    #[test]
    fn set_no_mailer() {
        let mut default_service = Service::default();

        assert_eq!(default_service.mailer, true);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_no_mailer(true);

        assert_eq!(service.mailer, false);
        assert!(service.deps.iter().all(|d| !d.is_mailer()));
    }

    #[test]
    fn set_no_messaging() {
        let mut default_service = Service::default();

        assert_eq!(default_service.messaging, true);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_no_messaging(true);

        assert_eq!(service.messaging, false);
        assert!(service.deps.iter().all(|d| !d.is_messaging()));
    }

    #[test]
    fn set_no_monitoring() {
        let mut default_service = Service::default();

        assert_eq!(default_service.monitoring, true);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_no_monitoring(true);

        assert_eq!(service.monitoring, false);
        assert!(service.deps.iter().all(|d| !d.is_monitoring()));
    }

    #[test]
    fn set_rest_protocol() {
        let mut default_service = Service::default();

        assert_eq!(default_service.protocol, Protocol::Rest);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps);

        // IMPROVEME on
        assert!(service.deps.iter().any(|d| d.is_graphql()));
    }

    #[test]
    fn set_grpc_protocol() {
        let mut default_service = Service::default();

        assert_eq!(default_service.protocol, Protocol::Rest);

        let path = Path::new(CARGO_ROOT).join("priv");
        let deps = de::parse_deps(path.as_path()).unwrap();
        let service = default_service.set_deps(deps).set_protocol(Protocol::Grpc);

        assert_eq!(service.protocol, Protocol::Grpc);
        assert!(service.deps.iter().any(|d| d.is_grpc()));
    }

    #[test]
    fn set_ssh() {
        let mut default_service = Service::default();

        assert_eq!(default_service.ssh, false);

        let service = default_service.set_ssh(true);

        assert_eq!(service.ssh, true);
    }
}
