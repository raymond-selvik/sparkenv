use std::io;

use clap::Parser;

mod cli;
mod spark_downloader;
mod app_config;
mod file_utils;

use app_config::*;

fn main() {

    let mut app_config = match open_configuration() {
        Some(x) => x,
        None => {
            println!("Did not found default config");
            create_new_configuration()
        }
    };


    /*dbg!(&app_config);
    app_config.add_spark_version("3.1");
    app_config.add_spark_version("3.2");
    app_config.add_spark_version("3.3");
    app_config.add_spark_version("3.3");
    dbg!(&app_config);

    let versions = app_config.get_installed_spark_versions();
    spark_downloader::get_versions();
    spark_downloader::get_version_option("spark-3.4.0");
    
    //spark_downloader::download_version("spark-3.3.3");

    dbg!(versions);*/
    


    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Versions =>  {
            println!("List versions")
        },         
        cli::Commands::Install(args) =>  {
            if app_config.is_installed(&args.version){
                println!("{} is already installed", &args.version);
                return
            }


            println!("Install spark version {:?}", args.version);
            let options = spark_downloader::get_version_option(&args.version);

            for (count, v) in options.iter().enumerate() {
                println!("{} \t {}", count, v);
            }

            println!("choose option");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let trimmed = input.trim();
            let option: usize = trimmed.parse().unwrap();

            println!("You chosed {}",options[option]);

            spark_downloader::download_version(&args.version,&options[option]);



        }
        cli::Commands::List => {
            spark_downloader::get_versions();
        },
    }
}
