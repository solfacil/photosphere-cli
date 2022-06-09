use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use std::{io::Error, path::Path, process::Command};
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    cmd: Commands,
    #[clap(long)]
    ssh: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Service(Service),
}

#[derive(Args)]
#[clap(args_conflicts_with_subcommands = true)]
struct Service {
    #[clap(subcommand)]
    cmd: ServiceCommand,
}

#[derive(Subcommand)]
#[clap(arg_required_else_help = true)]
enum ServiceCommand {
    New {
        #[clap(parse(try_from_str=validate_project_name))]
        path: String,
    },
}

const REPO_NAME: &'static &str = &"service-template";
const HTTPS_ENDPOINT: &'static &str = &"https://github.com/solfacil/";
const SSH_ENDPOINT: &'static &str = &"git@github.com:solfacil/";
const WITH_SPACE_DEFAULT: &'static &str = &"Service Template";
const SNAKE_CASE_DEFAULT: &'static &str = &"service_template";
const PASCAL_CASE_DEFAULT: &'static &str = &"ServiceTemplate";
const KABEB_CASE_DEFAULT: &'static &str = REPO_NAME;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Service(service) => match &service.cmd {
            ServiceCommand::New { path } => create_service(path, cli.ssh)?,
        },
    }

    Ok(())
}

fn create_service(path: &str, ssh: bool) -> Result<()> {
    let repo_url = get_repo_url(ssh);
    clone_repository(&repo_url, path)?;

    let project_name = get_project_name(path);
    setup_service(project_name, path)?;

    println!(
        "\u{001b}[32m \nGenerated {} with Photosphere {} \u{001b}[0m\n\n\
         Next:\n\
         $ cd {}\n\
         $ mix setup - to get dependencies\n\
         $ iex -S mix phx.server - to set up the service server",
        project_name, VERSION, path
    );

    println!(
        "\u{001b}[33m\nDon't forget to update {}/README.md! \u{001b}[33m",
        path
    );

    Ok(())
}

fn validate_project_name(path: &str) -> Result<String> {
    let name = get_project_name(path);
    let is_same_default = name.to_lowercase().eq(SNAKE_CASE_DEFAULT);
    let has_hifen = name.contains('-');
    let is_valid_name = name.chars().all(is_lower_alphanumeric);

    match (is_same_default, has_hifen, is_valid_name) {
        (true, _, _) => {
            bail!(
                "Hey...that's my name! Please name your project something other than {}.",
                name
            )
        }
        (_, true, _) => bail!("Please use snake_case for your project name."),
        (_, _, false) => {
            bail!("The project name can only contain lower alphanumeric characters and underscore.")
        }
        _ => Ok(path.to_string()),
    }
}

fn is_lower_alphanumeric(ch: char) -> bool {
    (ch.is_alphanumeric() && ch.is_lowercase()) || ch.eq(&'_')
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
        return <&str>::clone(SSH_ENDPOINT).to_owned() + REPO_NAME;
    }

    <&str>::clone(HTTPS_ENDPOINT).to_owned() + REPO_NAME
}

fn setup_service(name: &str, dest: &str) -> Result<()> {
    clean_source(dest)?;
    rename_source(name, dest)?;

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

        let kabeb_name = new.replace("_", "-");

        let with_space = new
            .split('_')
            .collect::<Vec<&str>>()
            .iter()
            .map(capitalize)
            .collect::<Vec<String>>()
            .join(" ");

        let pascal_name = new
            .split('_')
            .collect::<Vec<&str>>()
            .iter()
            .map(capitalize)
            .collect::<Vec<String>>()
            .join("");

        let new_data = data
            .replace(SNAKE_CASE_DEFAULT, new) // new name is already snake_case
            .replace(WITH_SPACE_DEFAULT, &with_space)
            .replace(KABEB_CASE_DEFAULT, &kabeb_name)
            .replace(PASCAL_CASE_DEFAULT, &pascal_name);

        std::fs::write(entry.path(), new_data)?;
    }

    Ok(())
}

fn capitalize(str: &&str) -> String {
    let mut chars = str.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn get_project_name(path: &str) -> &str {
    let path_entries = path.split('/').collect::<Vec<&str>>();

    path_entries.last().unwrap_or(REPO_NAME)
}
