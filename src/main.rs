use crate::general::file::sha256;
use crate::minecraft::papermc;
use crate::minecraft::papermc::build::build_json::Build;

pub mod request;
pub mod minecraft;
pub mod version;
pub mod general;

#[tokio::main]
async fn main() {
    let projects = papermc::check_projects().await.unwrap_or(vec![]);

    for entry in projects {
        let latest = entry.get_build().await;
        if latest.is_none() { 
            continue
        }
        
        let latest = latest.unwrap();
        let downloadable = latest.downloads.server_jar;
        
        let (_, path) = downloadable.download_with_name(format!("test/{}.jar", entry.project.name).as_str()).await.inspect(|file| {
            println!("{:?}", file);
        }).unwrap();
        
        println!("{:?}", downloadable.checksums);
        println!("{:?}", sha256(path).unwrap());
    }

    return;
}
