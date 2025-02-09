use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::{env, fs};

pub fn get_file_path(file_name: &str) -> Result<PathBuf, std::io::Error> {
    // Returns file path of given file name
    let current_dir = env::current_dir()?;
    let path = current_dir.join(file_name);
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("{:?} cannot be found: no such file or directory", path),
        ))
    }
}

pub fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    // reads file from filename
    let file_path = get_file_path(file_name)?;

    let file_contents = fs::read_to_string(file_path)?;

    Ok(file_contents)
}
