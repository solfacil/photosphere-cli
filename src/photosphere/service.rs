pub struct Service {
    deps: Vec<Dep>,
    auth: bool, // both authentication and authorization
    database: bool,
    monitoring: bool,
    mailer: bool,
    http_client: bool,
    graphql: bool,
    gettext: bool,
    messaging: bool,
    protocol: Protocol,
}

struct Dep {
    name: String,
    version: Option<String>,
    git: bool,
}

enum Protocol {
    Rest,
    Grpc,
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
    fn new(deps: Vec<Dep>) -> Service {
        Service {
            deps,
            auth: true,
            database: true,
            gettext: true,
            graphql: true,
            http_client: true,
            mailer: true,
            monitoring: true,
            messaging: true,
            protocol: Protocol::Rest,
        }
    }

    fn set_no_auth(&mut self) {
        self.auth = false;
        self.deps.retain(|d| !d.is_auth());
    }

    fn set_no_database(&mut self) {
        self.database = false;
        self.deps.retain(|d| !d.is_database());
    }

    fn set_no_gettext(&mut self) {
        self.gettext = false;
        self.deps.retain(|d| !d.is_gettext());
    }

    fn set_no_graphql(&mut self) {
        self.graphql = false;
        self.deps.retain(|d| !d.is_graphql());
    }

    fn set_no_grpc(&mut self) {
        // TODO remove all grpc stuff
        self.deps.retain(|d| !d.is_grpc());
    }

    fn set_no_http_client(&mut self) {
        self.http_client = false;
        self.deps.retain(|d| !d.is_http_client());
    }

    fn set_no_mailer(&mut self) {
        self.mailer = false;
        self.deps.retain(|d| !d.is_mailer());
    }

    fn set_no_messaging(&mut self) {
        self.messaging = false;
        self.deps.retain(|d| !d.is_messaging());
    }

    fn set_no_monitoring(&mut self) {
        self.monitoring = false;
        self.deps.retain(|d| !d.is_monitoring());
    }

    fn set_no_rest(&mut self) {
        // TODO remove all phoenix stuff
        self.set_no_graphql();
    }

    fn set_protocol(&mut self, protocol: Protocol) {
        match protocol {
            Protocol::Rest => {
                self.protocol = protocol;
                self.set_no_grpc();
            }
            Protocol::Grpc => {
                self.protocol = protocol;
                self.set_no_rest();
            }
        }
    }

    fn add_grpc_deps(&mut self) {
        // self.deps = vec![Dep{}];

        ()
    }
}
