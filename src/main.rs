use clap::Parser;

mod cli;
mod spark_downloader;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Versions =>  {
            println!("List versions")
        },         
        cli::Commands::Install(args) =>  {
            println!("Install spark version {:?}", args)
        }
        cli::Commands::List => {
            spark_downloader::get_versions().await;
        },
    }
}
