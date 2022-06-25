use crate::Protocol;

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

struct Dep {
    name: String,
    version: Option<String>,
    git: bool,
}

const GIT_URL: &'static &str = &"git@github.com:solfacil/REPLACE.git";

impl Dep {
    fn new(name: String, version: Option<String>, git: bool) -> Dep {
        Dep { name, version, git }
    }

    fn is_auth(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("guardian") || lc_name.eq("bodyguard")
    }

    fn is_database(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("database")
            || lc_name.eq("ecto_sql")
            || lc_name.eq("phoenix_ecto")
            || lc_name.eq("postgrex")
    }

    fn is_gettext(&self) -> bool {
        self.name.to_lowercase().eq("gettext")
    }

    fn is_graphql(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("absinthe")
            || lc_name.eq("absinthe_plug")
            || lc_name.eq("absinthe_phoenix")
            || lc_name.eq("absinthe_relay")
    }

    fn is_grpc(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("grpc") || lc_name.eq("protobuf") || lc_name.eq("google_protos")
    }

    fn is_http_client(&self) -> bool {
        self.name.to_lowercase().eq("http_client")
    }

    fn is_mailer(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("swoosh") || lc_name.eq("gen_smtp")
    }

    fn is_messaging(&self) -> bool {
        self.name.to_lowercase().eq("messaging")
    }

    fn is_monitoring(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.contains("spandex") || lc_name.eq("prom_ex") || lc_name.eq("ex_coveralls")
    }
}

impl Service {
    fn new() -> &'static mut Service {
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

    fn set_auth(&mut self, auth: bool) -> &mut Service {
        if !auth {
            self.auth = false;
            self.deps.retain(|d| !d.is_auth());

            return self;
        }

        self
    }

    fn set_deps(&mut self, deps: Vec<Dep>) -> &mut Service {
        self.deps = deps;

        self
    }

    fn set_database(&mut self, database: bool) -> &mut Service {
        if !database {
            self.database = false;
            self.deps.retain(|d| !d.is_database());

            return self;
        }

        self
    }

    fn set_gettext(&mut self, gettext: bool) -> &mut Service {
        if !gettext {
            self.gettext = false;
            self.deps.retain(|d| !d.is_gettext());

            return self;
        }

        self
    }

    fn set_graphql(&mut self, graphql: bool) -> &mut Service {
        if !graphql {
            self.graphql = false;
            self.deps.retain(|d| !d.is_graphql());

            return self;
        }

        self
    }

    fn set_http_client(&mut self, http_client: bool) -> &mut Service {
        if !http_client {
            self.http_client = false;
            self.deps.retain(|d| !d.is_http_client());

            return self;
        }

        self
    }

    fn set_mailer(&mut self, mailer: bool) -> &mut Service {
        if !mailer {
            self.mailer = false;
            self.deps.retain(|d| !d.is_mailer());

            return self;
        }

        self
    }

    fn set_messaging(&mut self, messaging: bool) -> &mut Service {
        if !messaging {
            self.messaging = false;
            self.deps.retain(|d| !d.is_messaging());

            return self;
        }

        self
    }

    fn set_monitoring(&mut self, monitoring: bool) -> &mut Service {
        if !monitoring {
            self.monitoring = false;
            self.deps.retain(|d| !d.is_monitoring());

            return self;
        }

        self
    }

    fn set_protocol(&mut self, protocol: Protocol) -> &mut Service {
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

    fn set_ssh(&mut self, ssh: bool) -> &mut Service {
        // SSH is disabled by default
        if ssh {
        self.ssh = true;

        return self;
        }

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
