use super::{service::Service, str_utils, validations::get_project_name};
use crate::ServiceArgs;
use anyhow::Result;
use std::{io::Error, path::Path, process::Command};
use walkdir::WalkDir;

const WITH_SPACE_DEFAULT: &'static &str = &"Service Template";
const PASCAL_CASE_DEFAULT: &'static &str = &"ServiceTemplate";
const KABEB_CASE_DEFAULT: &'static &str = REPO_NAME;
const REPO_NAME: &'static &str = &"service-template";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const HTTPS_URL: &'static &str = &"https://github.com/solfacil/service-template";
const SSH_URL: &'static &str = &"git@github.com:solfacil/service-template";

pub const SNAKE_CASE_DEFAULT: &'static &str = &"service_template";

pub fn build_service(args: &ServiceArgs) -> Service {
    let mut default_service = Service::new();

    let service_name = get_project_name(&args.path);
    let service_path = (*args.path).to_string();

    // set deps first to filter them after
    let service = default_service
        .set_deps(vec![])
        .set_auth(args.no_auth)
        .set_database(args.no_database)
        .set_gettext(args.no_gettext)
        .set_graphql(args.no_graphql)
        .set_http_client(args.no_http_client)
        .set_mailer(args.no_mailer)
        .set_messaging(args.no_messaging)
        .set_monitoring(args.no_monitoring)
        .set_name(String::from(service_name))
        .set_path(service_path)
        .set_protocol(args.protocol)
        .set_ssh(args.ssh);

    service.clone()
}

pub fn create_service(service: &Service) -> Result<()> {
    let repo_url = get_repo_url(service.ssh);
    clone_repository(&repo_url, &service.path)?;

    setup_service(&service)?;

    println!(
        "\u{001b}[32m \nGenerated {} with Photosphere {} \u{001b}[0m\n\n\
         Next:\n\
         $ cd {}\n\
         $ mix setup - to get dependencies\n\
         $ iex -S mix phx.server - to set up the service server",
        service.name, VERSION, service.path
    );

    println!(
        "\u{001b}[33m\nDon't forget to update {}/README.md! \u{001b}[33m",
        service.path
    );

    Ok(())
}

fn clone_repository(url: &str, dest: &str) -> Result<(), Error> {
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dest)
        .status()?;

    Ok(())
}

fn get_repo_url(is_ssh: bool) -> String {
    if is_ssh {
        return SSH_URL.to_string();
    }

    HTTPS_URL.to_string()
}

fn setup_service(service: &Service) -> Result<()> {
    clean_source(&service.path)?;
    rename_source(&service.name, &service.path)?;

    Ok(())
}

fn clean_source(dest: &str) -> Result<(), std::io::Error> {
    let path_to_rm = Path::new(dest).join(".git");

    std::fs::remove_dir_all(path_to_rm)
}

fn rename_source(new: &str, dest: &str) -> Result<()> {
    let entries = WalkDir::new(dest)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir());

    for entry in entries {
        let data = std::fs::read_to_string(entry.path())?;

        let kabeb_name = str_utils::to_kebab_case(new);
        let with_space = str_utils::to_title(new);
        let pascal_name = str_utils::to_pascal_case(new);

        let new_data = data
            .replace(SNAKE_CASE_DEFAULT, new) // new name is already snake_case
            .replace(WITH_SPACE_DEFAULT, &with_space)
            .replace(KABEB_CASE_DEFAULT, &kabeb_name)
            .replace(PASCAL_CASE_DEFAULT, &pascal_name);

        std::fs::write(entry.path(), new_data)?;
    }

    Ok(())
}
