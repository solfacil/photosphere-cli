#[derive(Clone, Debug, Default, PartialEq, Eq)]
// `Option` field for those
// that cannot be explicit on `mix.exs`
// but also can be `false`
pub struct Dep {
    pub(super) conflict: Option<bool>,
    pub(super) envs: Option<Vec<Env>>,
    pub(super) git: bool,
    pub(super) name: String,
    pub(super) runtime: Option<bool>,
    pub(super) version: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Env {
    Dev,
    Prod,
    Test,
}

impl Dep {
    pub fn new() -> Self {
        Dep::default()
    }

    pub fn is_auth(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("guardian") || lc_name.eq("bodyguard")
    }

    pub fn is_database(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("database")
            || lc_name.eq("ecto_sql")
            || lc_name.eq("phoenix_ecto")
            || lc_name.eq("postgrex")
    }

    pub fn is_gettext(&self) -> bool {
        self.name.to_lowercase().eq("gettext")
    }

    pub fn is_graphql(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("absinthe")
            || lc_name.eq("absinthe_plug")
            || lc_name.eq("absinthe_phoenix")
            || lc_name.eq("absinthe_relay")
    }

    pub fn is_grpc(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("grpc") || lc_name.eq("protobuf") || lc_name.eq("google_protos")
    }

    pub fn is_http_client(&self) -> bool {
        self.name.to_lowercase().eq("http_client")
    }

    pub fn is_mailer(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.eq("swoosh") || lc_name.eq("gen_smtp")
    }

    pub fn is_messaging(&self) -> bool {
        self.name.to_lowercase().eq("messaging")
    }

    pub fn is_monitoring(&self) -> bool {
        let lc_name = self.name.to_lowercase();

        lc_name.contains("spandex") || lc_name.eq("prom_ex") || lc_name.eq("ex_coveralls")
    }

    pub fn set_conflict(&mut self, conflict: Option<bool>) -> &mut Dep {
        self.conflict = conflict;

        self
    }

    pub fn set_envs(&mut self, envs: Option<Vec<Env>>) -> &mut Dep {
        self.envs = envs;

        self
    }

    pub fn set_git(&mut self, is_git: bool) -> &mut Dep {
        self.git = is_git;

        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Dep {
        self.name = name;

        self
    }

    pub fn set_runtime(&mut self, runtime: Option<bool>) -> &mut Dep {
        self.runtime = runtime;

        self
    }

    pub fn set_version(&mut self, version: String) -> &mut Dep {
        self.version = version;

        self
    }
}
