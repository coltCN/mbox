use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(version ,author,about,long_about=None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Action {
    Run(RunArgs),
}
#[derive(Parser, Debug, Clone)]
pub(crate) struct RunArgs {
    #[clap(short, long, value_parser)]
    pub config: String,
}
