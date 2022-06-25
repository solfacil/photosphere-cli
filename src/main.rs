use crate::photosphere::{setup, validations::validate_project_name};
use anyhow::Result;
use clap::{Args, Parser, Subcommand};

pub mod photosphere;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Service(service) => match &service.cmd {
            ServiceCommand::New { path } => setup::create_service(path, cli.ssh)?,
        },
    }

    Ok(())
}
