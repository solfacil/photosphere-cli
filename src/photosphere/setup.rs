use super::{service, service::Service, str_utils, validations::get_project_name};
use crate::ServiceArgs;
use anyhow::Result;
use std::{io::Error, path::Path, process::Command};
use walkdir::WalkDir;

const WITH_SPACE_DEFAULT: &'static &str = &"Service Template";
const PASCAL_CASE_DEFAULT: &'static &str = &"ServiceTemplate";
const KEBAB_CASE_DEFAULT: &'static &str = REPO_NAME;
const REPO_NAME: &'static &str = &"service-template";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const HTTPS_URL: &'static &str = &"https://github.com/solfacil/service-template";
const SSH_URL: &'static &str = &"git@github.com:solfacil/service-template";

pub const SNAKE_CASE_DEFAULT: &'static &str = &"service_template";

pub fn build_partial_service(service_path: &str, is_ssh: bool) -> Service {
    let service_name = get_project_name(service_path);

    let mut default_service = Service::new(service_name, service_path);

    default_service.set_ssh(is_ssh).clone()
}

pub fn create_service(service: &mut Service, args: &ServiceArgs) -> Result<()> {
    let repo_url = get_repo_url(service.ssh);
    clone_repository(&repo_url, &service.path)?;

    setup_service(service, args)?;

    println!(
        "\u{001b}[32m \nGenerated {} with Photosphere {} \u{001b}[0m\n\n\
         Next:\n\
         $ cd {}\n\
         $ mix setup - to get dependencies\n\
         $ iex -S mix phx.server - to set up the service server",
        service.name,
        VERSION,
        service.path.as_path().display()
    );

    println!(
        "\u{001b}[33m\nDon't forget to update {}/README.md! \u{001b}[33m",
        service.path.as_path().display()
    );

    Ok(())
}

fn clone_repository(url: &str, dest: &Path) -> Result<(), Error> {
    let dest_os = dest.as_os_str();

    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dest_os)
        .arg("-b")
        .arg("photosphere-test")
        .status()?;

    Ok(())
}

fn setup_service(service: &mut Service, args: &ServiceArgs) -> Result<()> {
    let deps = service::de::parse_deps(service)?;

    // set deps first to filter them after
    service
        .set_deps(deps)
        .set_no_auth(args.no_auth)
        .set_no_database(args.no_database)
        .set_no_graphql(args.no_graphql)
        .set_no_http_client(args.no_http_client)
        .set_no_mailer(args.no_mailer)
        .set_no_messaging(args.no_messaging)
        .set_no_monitoring(args.no_monitoring)
        .set_protocol(args.protocol);

    let root_path = service.path.as_path();

    // We don't need our `service_template` commit history anymore
    let git_path = root_path.join(".git");
    std::fs::remove_dir_all(git_path)?;

    // Also we don't need old `mix.lock`
    let lock_path = root_path.join("mix.lock");
    std::fs::remove_file(lock_path)?;

    rename_source(service)?;

    apply_config(service)?;

    Ok(())
}

fn get_repo_url(is_ssh: bool) -> String {
    if is_ssh {
        return SSH_URL.to_string();
    }

    HTTPS_URL.to_string()
}

fn apply_config(service: &Service) -> Result<()> {
    if !service.auth {
        service::ser::nuke_auth(service)?;
    }

    if !service.database {
        service::ser::nuke_database(service)?;
    }

    if !service.graphql || service.protocol.is_grpc() {
        service::ser::nuke_graphql(service)?;
    }

    if service.protocol.is_rest() {
        service::ser::nuke_grpc(service)?;
    }

    if !service.http_client {
        service::ser::nuke_http_client(service)?;
    }

    if !service.mailer {
        service::ser::nuke_mailer(service)?;
    }

    if !service.messaging {
        service::ser::nuke_messaging(service)?;
    }

    if !service.monitoring {
        service::ser::nuke_monitoring(service)?;
    }

    if !service.protocol.is_rest() {
        service::ser::nuke_rest(service)?;
    }

    service::ser::dump_deps(service)?;

    Ok(())
}

fn rename_source(service: &Service) -> Result<()> {
    let file_entries = WalkDir::new(&service.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir());

    for entry in file_entries {
        let data = std::fs::read_to_string(entry.path())?;

        let kabeb_name = str_utils::to_kebab_case(&service.name);
        let with_space = str_utils::to_title(&service.name);
        let pascal_name = str_utils::to_pascal_case(&service.name);

        let new_data = data
            .replace(SNAKE_CASE_DEFAULT, &service.name) // new name is already snake_case
            .replace(WITH_SPACE_DEFAULT, &with_space)
            .replace(KEBAB_CASE_DEFAULT, &kabeb_name)
            .replace(PASCAL_CASE_DEFAULT, &pascal_name);

        std::fs::write(entry.path(), new_data)?;
    }

    std::fs::rename(
        service.path.as_path().join("lib").join(SNAKE_CASE_DEFAULT),
        service.path.as_path().join("lib").join(&service.name),
    )?;
    std::fs::rename(
        service
            .path
            .as_path()
            .join("lib")
            .join(format!("{}_web", SNAKE_CASE_DEFAULT)),
        service
            .path
            .as_path()
            .join("lib")
            .join(format!("{}_web", &service.name)),
    )?;

    std::fs::rename(
        service
            .path
            .as_path()
            .join("lib")
            .join(format!("{}.ex", SNAKE_CASE_DEFAULT)),
        service
            .path
            .as_path()
            .join("lib")
            .join(format!("{}.ex", &service.name)),
    )?;
    std::fs::rename(
        service
            .path
            .as_path()
            .join("lib")
            .join(format!("{}_web.ex", SNAKE_CASE_DEFAULT)),
        service
            .path
            .as_path()
            .join("lib")
            .join(format!("{}_web.ex", &service.name)),
    )?;

    Ok(())
}
