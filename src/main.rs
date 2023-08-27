use clap::Parser;

mod cli;
mod spark_downloader;
mod app_config;

#[tokio::main]
async fn main() {
    println!("aksdkløds");
    let mut app_config = app_config::app_config::AppConfig::init();
    dbg!(&app_config);
    app_config.add_spark_innstallation();

    dbg!(app_config);
    


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
