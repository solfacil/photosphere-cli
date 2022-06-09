use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use std::{
    io::Error,
    path::{Path, PathBuf},
    process::Command,
};
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
const SNAKE_CASE_DEFAULT: &'static &str = &"service_template";
const CAMMEL_CASE_DEFAULT: &'static &str = &"ServiceTemplate";
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

    Ok(())
}

fn validate_project_name(name: &str) -> Result<String> {
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
        _ => return Ok(name.to_string()),
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
