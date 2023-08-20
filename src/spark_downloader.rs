use reqwest;
use select::{document::Document, predicate::Name};


pub async fn get_versions() {
    let res = reqwest::get("https://archive.apache.org/dist/spark/").await.unwrap().text().await.unwrap();

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
