use crate::Protocol;
use dep::Dep;

pub mod dep;

pub struct Service {
    auth: bool, // both authentication and authorization
    database: bool,
    deps: Vec<Dep>,
    gettext: bool,
    graphql: bool,
    http_client: bool,
    mailer: bool,
    messaging: bool,
    monitoring: bool,
    protocol: Protocol,
    ssh: bool,
}

const GIT_URL: &'static &str = &"git@github.com:solfacil/REPLACE.git";

impl Service {
    pub fn new() -> &'static mut Service {
        &mut Service {
            auth: true,
            database: true,
            deps: vec![],
            gettext: true,
            graphql: true,
            http_client: true,
            mailer: true,
            monitoring: true,
            messaging: true,
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
