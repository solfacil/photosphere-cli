pub struct Dep {
    name: String,
    version: Option<String>,
    git: bool,
}

impl Dep {
    pub fn new(name: String, version: Option<String>, git: bool) -> Dep {
        Dep { name, version, git }
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
}
