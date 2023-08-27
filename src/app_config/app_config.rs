use std::collections::HashMap;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::ptr::null;

use directories::BaseDirs;
use serde::ser::SerializeTupleVariant;
use serde::{Serialize, Deserialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    sparkenv_path: PathBuf,
    spark_installations: HashMap<String, SparkInstallation>,
    active_installation: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct SparkInstallation {
    version: String,
    path: PathBuf
}

impl AppConfig {
    pub fn init() -> Self{
        let Some(base_dir) = BaseDirs::new() else {
            panic!("Could not find base dir")
        };

        let sparkenv_path = base_dir.home_dir().join(".sparkenv");

        if !Path::exists(&sparkenv_path) {
            println!("Could not find exisitng configuration. Creating new one");
            create_new_configuration(sparkenv_path)
        }
        else {
            println!("Found existing configuration");
            open_configuration(sparkenv_path)
        }
        
    }

    pub fn add_spark_innstallation(&mut self) {
        let spark_inst = SparkInstallation{
            version: "3.2".to_owned(), 
            path: Path::new("test").to_owned()
        };

        self.spark_installations.insert("spark3.2".to_owned(), spark_inst);
        self.update_config_file();

    }

    fn update_config_file(&self) {
        let f = OpenOptions::new()
        .write(true)
        .open(self.sparkenv_path.join("config.json")).unwrap();

        serde_json::to_writer(f, &self).unwrap();
    }

}

fn create_new_configuration(sparkenv_path: PathBuf) -> AppConfig {
    fs::create_dir(&sparkenv_path).unwrap();

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(sparkenv_path.join("config.json")).unwrap();

    let config = AppConfig {
        sparkenv_path: sparkenv_path,
        spark_installations: HashMap::new(),
        active_installation: None
    };

    serde_json::to_writer(f, &config);

    return config
}

fn open_configuration(sparkenv_path: PathBuf) -> AppConfig {
    let f = File::open(sparkenv_path.join("config.json")).unwrap();

    let config: AppConfig = serde_json::from_reader(f).unwrap();

    return config;
}



