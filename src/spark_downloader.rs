use std::{fmt::format, fs::File, path::{Path, PathBuf}};
use flate2::read::GzDecoder;
use reqwest::blocking;
use select::{document::Document, predicate::Name};
use tar::Archive;


use crate::app_config::{app_config::AppConfig, self};


pub fn get_versions() {
    let res = reqwest::blocking::get("https://archive.apache.org/dist/spark/").unwrap().text().unwrap();

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x|  {
            if x.contains("spark") {
                let mut spark_version = String::from(x);
                spark_version.pop();
                println!("{}", spark_version);
            }
            
        });
}


pub fn get_version_option(version: &str) -> Vec<String> {
    let url = format!("https://archive.apache.org/dist/spark/{version}", version=version);

    let res = reqwest::blocking::get(&url).unwrap().text().unwrap();

    let mut version_options: Vec<String> = Vec::new();

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x|  {
            if x.ends_with("tgz") {
                version_options.push(x.to_owned());
            }
        });
    return  version_options;
}
pub fn download_version(version: &str, option: &str) {
        let url = get_download_link(version);
        let  mut resp = reqwest::blocking::get(&url).unwrap();

        dbg!(&url);



        let dest_path: PathBuf  = app_config::app_config::get_sparkenv_path().join(format!("versions/{}", option));
        let  mut f = File::create(&dest_path).expect("failed to create file");

        println!("{}", url);

        std::io::copy(&mut resp, &mut f);

        unzip_file(dest_path);
      
}

fn unzip_file(path: PathBuf) {
    let mut filepath = path.clone();
    dbg!(&path);

    let tar_gz = File::open(&path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    let dir_name = path.file_stem().unwrap();
    let p = filepath.pop();

    dbg!(dir_name);
    dbg!(p);
    //let stem = p.file_stem().unwrap().to_owned();

    //dbg!(&stem);
    //archive.unpack(stem).unwrap();
}

fn get_download_link(version: &str ) -> String {
    let url = format!("https://archive.apache.org/dist/spark/{version}/{version}-bin-hadoop3.tgz", version=version);
    return url;
}


#[cfg(test)]
mod tests {
    use crate::spark_downloader::get_download_link;

    #[test]
    fn test_get_download_link() {
        let expected = "https://archive.apache.org/dist/spark/spark-3.3.0/spark-3.3.0-bin-hadoop3.tgz".to_owned();
        let version = "spark-3.3.0";

        let actual = get_download_link(version);

        assert_eq!(expected, actual);
    }
}