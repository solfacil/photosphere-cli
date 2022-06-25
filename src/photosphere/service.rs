use crate::{setup::SNAKE_CASE_DEFAULT, Protocol};
use dep::Dep;
use std::path::PathBuf;

pub mod dep;

#[derive(Clone)]
pub struct Service {
    pub(super) auth: bool, // both authentication and authorization
    pub(super) database: bool,
    pub(super) deps: Vec<Dep>,
    pub(super) gettext: bool,
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

const GIT_URL: &'static &str = &"git@github.com:solfacil/REPLACE.git";

impl Service {
    pub fn default() -> Self {
        let default_path = PathBuf::from(&format!("./{}", SNAKE_CASE_DEFAULT));

        Service {
            auth: true,
            database: true,
            deps: vec![],
            gettext: true,
            graphql: true,
            http_client: true,
            mailer: true,
            monitoring: true,
            messaging: true,
            name: SNAKE_CASE_DEFAULT.to_string(),
            path: default_path,
            protocol: Protocol::Rest,
            ssh: false,
        }
    }

    pub fn set_auth(&mut self, no_auth: bool) -> &mut Service {
        if no_auth {
            self.auth = false;
            self.deps.retain(|d| !d.is_auth());

            return self;
        }

        self
    }

    pub fn set_deps(&mut self, deps: Vec<Dep>) -> &mut Service {
        self.deps = deps;

        self
    }

    pub fn set_database(&mut self, no_database: bool) -> &mut Service {
        if no_database {
            self.database = false;
            self.deps.retain(|d| !d.is_database());

            return self;
        }

        self
    }

    pub fn set_gettext(&mut self, no_gettext: bool) -> &mut Service {
        if no_gettext {
            self.gettext = false;
            self.deps.retain(|d| !d.is_gettext());

            return self;
        }

        self
    }

    pub fn set_graphql(&mut self, no_graphql: bool) -> &mut Service {
        if no_graphql {
            self.graphql = false;
            self.deps.retain(|d| !d.is_graphql());

            return self;
        }

        self
    }

    pub fn set_http_client(&mut self, no_http_client: bool) -> &mut Service {
        if no_http_client {
            self.http_client = false;
            self.deps.retain(|d| !d.is_http_client());

            return self;
        }

        self
    }

    pub fn set_mailer(&mut self, no_mailer: bool) -> &mut Service {
        if no_mailer {
            self.mailer = false;
            self.deps.retain(|d| !d.is_mailer());

            return self;
        }

        self
    }

    pub fn set_messaging(&mut self, no_messaging: bool) -> &mut Service {
        if no_messaging {
            self.messaging = false;
            self.deps.retain(|d| !d.is_messaging());

            return self;
        }

        self
    }

    pub fn set_monitoring(&mut self, no_monitoring: bool) -> &mut Service {
        if no_monitoring {
            self.monitoring = false;
            self.deps.retain(|d| !d.is_monitoring());

            return self;
        }

        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Service {
        self.name = name;

        self
    }

    pub fn set_path(&mut self, path: String) -> &mut Service {
        self.path = PathBuf::from(&path);

        self
    }

    pub fn set_protocol(&mut self, protocol: Protocol) -> &mut Service {
        match protocol {
            Protocol::Rest => {
                self.protocol = protocol;
                set_no_grpc(self);

                self
            }
            Protocol::Grpc => {
                self.protocol = protocol;
                set_no_rest(self);

                self
            }
        }
    }

    pub fn set_ssh(&mut self, ssh: bool) -> &mut Service {
        // SSH is disabled by default
        self.ssh = ssh;

        self
    }
}

fn set_no_grpc(service: &mut Service) -> &mut Service {
    // TODO remove all grpc stuff
    service.deps.retain(|d| !d.is_grpc());

    service
}

fn set_no_rest(service: &mut Service) -> &mut Service {
    // TODO remove all phoenix stuff
    service.set_graphql(false);

    service
}
