use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use sha2::{Digest, Sha256};


pub fn sha256(path: impl AsRef<Path>) -> io::Result<String> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(0))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}