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


    fn update_config_file(&self) {
        let f = OpenOptions::new()
        .write(true)
        .open(self.sparkenv_path.join("config.json")).unwrap();

        serde_json::to_writer(f, &self).unwrap();
    }

}



pub fn create_new_configuration() -> AppConfig {
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

pub fn open_configuration() -> Option<AppConfig> {
    let sparkenv_path = get_sparkenv_path();
    if !Path::exists(&sparkenv_path) {
        return None;
    }


    let f = File::open(sparkenv_path.join("config.json")).unwrap();

    let config: AppConfig = serde_json::from_reader(f).unwrap();

    return Some(config);
}

fn get_sparkenv_path() -> PathBuf {
    let Some(base_dir) = BaseDirs::new() else {
        panic!("Could not find base dir")
    };

    let sparkenv_path = base_dir.home_dir().join(".sparkenv");

    return sparkenv_path;

}



