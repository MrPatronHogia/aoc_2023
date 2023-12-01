use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use crate::prelude::Error;
use crate::prelude::Result;

pub fn read_file(file_name: &str) -> Result<String> {
    let file = open(file_name)?;
    let contents = read(file)?;

    Ok(contents)
}

fn read(file: File) -> Result<String> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(error) => Err(Error::Generic(error.to_string())),
    }
}

fn open(file_name: &str) -> Result<File> {
    let file_result = File::open(file_name);
    let file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(Error::Generic(error.to_string())),
    };

    Ok(file)
}
