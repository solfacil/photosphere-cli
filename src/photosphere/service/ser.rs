use super::{de::DEPS_START, dep::Env, Service};
use anyhow::Result;
use std::fs;

const GIT_URL: &'static &str = &"git@github.com:solfacil/PKG.git";

// OH GOSH we need a serious parser...
pub fn dump_deps(service: &Service) -> Result<()> {
    let mix_path = service.path.as_path().join("mix.exs");
    let mix_exs = fs::read_to_string(mix_path.clone())?;

    let mut deps_str = "".to_owned();

    for dep in service.deps.iter() {
        let base_str = r#"{:NAME, "~> VERSION""#.to_owned();
        let mut dep_str = base_str
            .replace("NAME", &dep.name)
            .replace("VERSION", &dep.version);

        if let Some(envs) = &dep.envs {
            let mut env_str = ", only: [".to_owned();

            for env in envs.iter() {
                if envs.len() == 1 {
                    env_str.push_str(env.to_str());
                    break;
                }

                let extra_env = &(env.to_str().to_owned() + ", ");
                env_str.push_str(extra_env);
            }

            env_str.push(']');
            dep_str.push_str(&env_str);
        }

        if let Some(runtime) = dep.runtime {
            dep_str.push_str(&format!(", runtime: {}", runtime));
        }

        if dep.git {
            let pkg_url = GIT_URL.replace("PKG", &dep.name);
            let git_str = r#", git: "URL""#;
            dep_str.push_str(&git_str.replace("URL", &pkg_url));

            let tag_str = r#", tag: "V""#;
            dep_str.push_str(&tag_str.replace('V', &dep.version));
        }

        if let Some(conflict) = dep.conflict {
            dep_str.push_str(&format!(", override: {}", conflict));
        }

        dep_str.push_str("}\n");
        deps_str.push_str(&dep_str);
    }

    fs::write(mix_path, mix_exs.replace(DEPS_START, &deps_str))?;

    Ok(())
}

pub fn nuke_auth(service: &Service) -> Result<()> {
    let root = service.path.as_path();

    let auth_path = root
        .join("lib")
        .join(format!("{}_web", service.name))
        .join("auth");
    fs::remove_dir_all(auth_path)?;

    let runtime_path = root.join("config").join("runtime.exs");
    let runtime = fs::read_to_string(runtime_path.clone())?;

    let no_auth: String = runtime
        .lines()
        .filter(|l| {
            !l.contains("Guardian")
                || !l.contains("GUARDIAN")
                || !l.contains("ttl")
                || !l.contains("issuer")
        })
        .collect::<Vec<&str>>()
        .join("\n");
    fs::write(runtime_path, no_auth)?;

    Ok(())
}

// FIXME - needs to also remove ecto lines
pub fn nuke_database(service: &Service) -> Result<()> {
    let root = service.path.as_path();

    let config_path = root.join("config");

    // Clean config files
    for c in fs::read_dir(config_path)?.filter_map(|f| f.ok()) {
        let data = fs::read_to_string(c.path())?;

        let new_data: String = data
            .lines()
            .filter(|l| is_database_text(l))
            .collect::<Vec<&str>>()
            .join("\n");

        fs::write(c.path(), new_data)?;
    }

    let telemetry_path = root
        .join("lib")
        .join(format!("{}_web", service.name))
        .join("telemetry.ex");
    let telemetry = fs::read_to_string(telemetry_path.clone())?;
    let tel_data: String = telemetry
        .lines()
        .filter(|l| is_database_text(l))
        .collect::<Vec<&str>>()
        .join("\n");
    fs::write(telemetry_path, tel_data)?;

    fs::remove_file(root.join("test").join("support").join("data_case.ex"))?;

    fs::remove_dir_all(root.join("priv").join("repo"))?;

    Ok(())
}

pub fn nuke_graphql(service: &Service) -> Result<()> {
    let web_path = service
        .path
        .as_path()
        .join("lib")
        .join(format!("{}_web", service.name));

    let endpoint_path = web_path.join("endpoint.ex");
    let endpoint = fs::read_to_string(endpoint_path.clone())?;

    fs::write(
        endpoint_path,
        endpoint
            .replace(", Absinthe.Plug.Parser", "")
            .lines()
            .filter(|l| !l.contains("Absinthe") || !l.contains("GraphQL"))
            .collect::<Vec<&str>>()
            .join("\n"),
    )?;

    fs::remove_dir_all(web_path.join("graphql"))?;

    Ok(())
}

pub fn nuke_grpc(service: &Service) -> Result<()> {
    let root = service.path.as_path();

    let web_path = root.join("lib").join(format!("{}_web", service.name));

    let config_path = root.join("config");

    // Clean config files
    for c in fs::read_dir(config_path)?.filter_map(|f| f.ok()) {
        let data = fs::read_to_string(c.path())?;

        let new_data: String = data
            .lines()
            .filter(|l| !l.contains("gRPC") || !l.contains("grpc"))
            .collect::<Vec<&str>>()
            .join("\n");

        fs::write(c.path(), new_data)?;
    }

    let flake_path = root.join("flake.nix");
    fs::write(
        flake_path.clone(),
        fs::read_to_string(flake_path)?
            .lines()
            .filter(|l| !l.contains("grpcurl") || !l.contains("protobuf"))
            .collect::<Vec<&str>>()
            .join("\n"),
    )?;

    let endpoint_path = web_path.join("endpoint.ex");
    fs::write(
        endpoint_path.clone(),
        fs::read_to_string(endpoint_path)?
            .lines()
            .filter(|l| !l.contains("GRPC"))
            .collect::<Vec<&str>>()
            .join("\n"),
    )?;

    fs::remove_dir_all(web_path.join("grpc"))?;

    Ok(())
}

pub fn nuke_http_client(service: &Service) -> Result<()> {
    let config_path = service.path.as_path().join("config").join("config.exs");

    fs::write(
        config_path.clone(),
        fs::read_to_string(config_path)?
            .lines()
            .filter(|l| !l.contains(":tesla"))
            .collect::<Vec<&str>>()
            .join("\n"),
    )?;

    Ok(())
}

pub fn nuke_mailer(service: &Service) -> Result<()> {
    let root = service.path.as_path();

    let web_path = root.join("lib").join(format!("{}_web", service.name));

    fs::remove_dir_all(web_path.join("mailer"))?;

    let runtime_path = root.join("config").join("runtime.exs");
    fs::write(
        runtime_path.clone(),
        fs::read_to_string(runtime_path)?
            .lines()
            .filter(|l| !l.contains("MAILER") || !l.contains("Mailer") || !l.contains("mailer"))
            .collect::<Vec<&str>>()
            .join("\n"),
    )?;

    fs::write(
        root.join(".env-sample"),
        fs::read_to_string(root.join(".env-sample"))?
            .lines()
            .filter(|l| !l.contains("MAILER"))
            .collect::<Vec<&str>>()
            .join("\n"),
    )?;

    Ok(())
}

pub fn nuke_messaging(service: &Service) -> Result<()> {
    let config_path = service.path.as_path().join("config");

    // Clean config files
    for c in fs::read_dir(config_path)?.filter_map(|f| f.ok()) {
        let data = fs::read_to_string(c.path())?;

        let new_data: String = data
            .lines()
            .filter(|l| !l.contains("messaging") || !l.contains("Messaging"))
            .filter(|l| {
                !l.contains("kafka_ex")
                    || !l.contains("brokers")
                    || !l.contains("disable_default_broker")
            })
            .collect::<Vec<&str>>()
            .join("\n");

        fs::write(c.path(), new_data)?;
    }

    Ok(())
}

// TODO
pub fn nuke_monitoring(_service: &Service) -> Result<()> {
    Ok(())
}

// TODO
pub fn nuke_rest(_service: &Service) -> Result<()> {
    Ok(())
}

fn is_database_text(line: &str) -> bool {
    line.contains("database")
        || line.contains("DATABASE")
        || line.contains("Repo")
        || line.contains("repo")
        || line.contains("POOL_SIZE")
        || line.contains("ECTO_IPV6")
        || line.contains("username")
        || line.contains("password")
        || line.contains("pool")
        || line.contains("pool_size")
        || line.contains("hostname")
        || line.contains("show_sensitive_data_on_connection_error")
}
