use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::{io::Error, process::Command};

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
    New { path: String },
}

const REPO_NAME: &'static &str = &"service-template";
const HTTPS_ENDPOINT: &'static &str = &"https://github.com/solfacil/";
const SSH_ENDPOINT: &'static &str = &"git@github.com:solfacil/";

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

fn clone_repository<'a>(url: &str, dest: &'a str) -> Result<&'a str, Error> {
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dest)
        .status()?;

    Ok(dest)
}

fn get_repo_url(is_ssh: bool) -> String {
    if is_ssh {
        return <&str>::clone(SSH_ENDPOINT).to_owned() + REPO_NAME;
    }

    <&str>::clone(HTTPS_ENDPOINT).to_owned() + REPO_NAME
}
