use std::{fs::File, path::{Path, PathBuf}};

use flate2::{write as arch_writer, Compression};
use flate2::read as arch_reader;

pub fn unzip_file(path: PathBuf) {

}

pub fn zip_file(path: PathBuf) {
    let tar_gz = File::create(path.with_extension("tgz")).unwrap();
    let enc = arch_writer::GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    
    if path.is_file() {
        tar.append_path(path);
    }
    else if path.is_dir() {
        tar.append_dir_all(".", path);  
    }

}



#[cfg(test)]
mod tests {
    use std::fs::create_dir;

    use super::*;
    
    #[test]
    fn archive_file() {
        let p = Path::new("tmp/test.txt");
        File::create(&p);
        zip_file(p.to_path_buf());
    }

    #[test]
    fn archive_dir() {
        let p = Path::new("tmp/test.txt");
        let dir = p.parent().unwrap();
        create_dir(dir);
        File::create(p);

        zip_file(dir.to_path_buf());




    }
}



