use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install Spark version
    Install(InstallArg),

    /// List available Spark versions to install
    List,

    /// List installed versions of Spark on computer
    Versions,


}

#[derive(Args, Debug)]
pub struct InstallArg {
    version: String
}