use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn read_file_as_string(path: &str) -> Result<String, Box<dyn Error>> {
    let path = get_path_from_root(path)?;
    let input = fs::read_to_string(path)?;
    Ok(input.trim().to_owned())
}

pub fn read_file_as_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = get_path_from_root(path)?;
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(str::to_owned).collect())
}

pub fn read_file_as_elements<T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let path = get_path_from_root(path)?;
    let content = fs::read_to_string(path)?;
    content.lines()
        .map(|s| s.parse()
            .map_err(|e| Box::<dyn Error>::from(format!("{:?}", e))))
        .collect()
}

fn get_path_from_root(path: &str) -> Result<PathBuf, String> {
    match Path::new(env!("CARGO_MANIFEST_DIR")).parent() {
        None => Err("Unable to get to root dir".to_owned()),
        Some(parent) => Ok(parent.join(path)),
    }
}

