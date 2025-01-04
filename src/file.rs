use std::{fs, io};
use std::io::{Read, Write};
use std::path::Path;

#[allow(dead_code)]
pub fn read_file(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    file.flush()?;
    Ok(data)
}

#[allow(dead_code)]
pub fn write_file(path: &Path, data: String) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(data.as_bytes())?;
    file.flush()?;
    Ok(())
}