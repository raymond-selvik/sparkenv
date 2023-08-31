use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::error::Error;


use directories::BaseDirs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    sparkenv_path: PathBuf,
    spark_installations: HashMap<String, PathBuf>,
    active_installation: Option<String>
}



impl AppConfig {

    pub fn add_spark_version(&mut self, version: &str) {
        let versions_path = get_sparkenv_path().join("versions");

        self.spark_installations.insert(version.to_owned(), versions_path.join(version));
        self.update_config_file();
    }

    pub fn get_installed_spark_versions(&self) -> Vec<String> {

        let mut versions: Vec<String>  = self.spark_installations.clone().into_iter()
            .map(|(version, path)| version)
            .collect();

        versions.sort();

        return versions;
    }

    pub fn is_installed(&self, version: &str) -> bool {
        self.spark_installations.contains_key(version)
    }


    fn update_config_file(&self) {
        let f = OpenOptions::new()
        .write(true)
        .open(self.sparkenv_path.join("config.json")).unwrap();

        serde_json::to_writer(f, &self).unwrap();
    }

}



pub fn get_or_create_configuration() -> Result<AppConfig, Box<dyn Error>> {
    let sparkenv_path = get_sparkenv_path();
    if !Path::exists(&sparkenv_path) {
        fs::create_dir(&sparkenv_path)?;
    }

    let config_file_path = sparkenv_path.join("config.json");
    let app_config =  match File::open(config_file_path) {
        Ok(f) => {
            serde_json::from_reader(f)
        },
        Err(_) => {
            Ok(create_config_file())
        },

    };

    Ok(app_config)
}

pub fn create_config_file() -> AppConfig {
    let sparkenv_path = get_sparkenv_path();
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



pub fn get_sparkenv_path() -> PathBuf {
    let Some(base_dir) = BaseDirs::new() else {
        panic!("Could not find base dir")
    };

    let sparkenv_path = base_dir.home_dir().join(".sparkenv");

    return sparkenv_path;

}



