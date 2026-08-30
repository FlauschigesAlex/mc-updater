use crate::minecraft::papermc::build::build_json::FileDownload;
use crate::request::get_request_bytes;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub mod build_json;

impl FileDownload {
    
    pub async fn download(&self) -> Result<(File, PathBuf), String> {
        self.download_with_name(&self.name).await
    }
    
    pub async fn download_with_name(&self, uri: &str) -> Result<(File, PathBuf), String> {
        let path = Path::new(uri);
        self.download_with_path(path).await
    }
    
    pub async fn download_with_path(&self, path: &Path) -> Result<(File, PathBuf), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        
        let file = File::create(path).map_err(|e| e.to_string())?;
        let file = self.download_with_file(file).await?;
        
        Ok((file, path.to_path_buf()))
    }
    
    async fn download_with_file(&self, mut file: File) -> Result<File, String> {
        let bytes = get_request_bytes(&self.url).await.map_err(|e| e.to_string())?;
        file.write_all(&bytes).map_err(|e| e.to_string())?;
        
        return Ok(file);
    }
}