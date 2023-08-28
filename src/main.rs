use clap::Parser;

mod cli;
mod spark_downloader;
mod app_config;

use app_config::app_config::*;

#[tokio::main]
async fn main() {

    let mut app_config = match open_configuration() {
        Some(x) => x,
        None => {
            println!("Did not found default config");
            create_new_configuration()
        }
    };


    dbg!(&app_config);
    app_config.add_spark_version("3.1");
    app_config.add_spark_version("3.2");
    app_config.add_spark_version("3.3");
    app_config.add_spark_version("3.3");
    dbg!(&app_config);

    let versions = app_config.get_installed_spark_versions();


    dbg!(versions);
    


    /*let cli = cli::Cli::parse();

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
    }*/
}
