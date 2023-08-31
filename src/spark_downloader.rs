use flate2::read::MultiGzDecoder;
use select::{document::Document, predicate::Name};
use tar::Archive;

use crate::app_config;


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
        let url = get_download_link(version, option);
        let  mut res = reqwest::blocking::get(&url).unwrap();

        dbg!(&url);

        let mut gz = MultiGzDecoder::new(res);

        let mut archive = Archive::new(gz);
        let path = app_config::get_sparkenv_path().join("versions").join(".");
        dbg!(&path);
        archive.unpack(path).unwrap();

}



fn get_download_link(version: &str, option: &str) -> String {
    let url = format!("https://archive.apache.org/dist/spark/{version}/{option}", version=version, option=option);
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