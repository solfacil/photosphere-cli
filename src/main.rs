use crate::photosphere::{setup, validations::validate_project_name};
use anyhow::Result;
use clap::{ArgEnum, Args, Parser, Subcommand};

pub mod parser;
pub mod photosphere;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    cmd: Commands,
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
    New(ServiceArgs),
}

#[derive(Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct ServiceArgs {
    #[clap(parse(try_from_str=validate_project_name))]
    path: String,
    #[clap(long)]
    ssh: bool,
    #[clap(long)]
    no_auth: bool,
    #[clap(long)]
    no_database: bool,
    #[clap(long)]
    no_graphql: bool,
    #[clap(long)]
    no_http_client: bool,
    #[clap(long)]
    no_mailer: bool,
    #[clap(long)]
    no_messaging: bool,
    #[clap(long)]
    no_monitoring: bool,
    #[clap(long, arg_enum, default_value_t = Protocol::Rest)]
    protocol: Protocol,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ArgEnum)]
pub enum Protocol {
    Rest,
    Grpc,
}

impl Protocol {
    pub fn is_rest(&self) -> bool {
        *self == Protocol::Rest
    }

    pub fn is_grpc(&self) -> bool {
        *self == Protocol::Grpc
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Service(service) => match &service.cmd {
            ServiceCommand::New(args) => {
                let mut service = setup::build_partial_service(&args.path, args.ssh);

                setup::create_service(&mut service, args)?
            }
        },
    }

    Ok(())
}
