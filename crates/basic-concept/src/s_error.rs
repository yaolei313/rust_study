use std::{error::Error, fmt::Display, fs::{self, File}, io::{self, Read}};
use std::num;

#[derive(Debug)]
pub struct AppError {
    code: i32,
    message: String
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, message: {}", self.code, self.message)
    }
}

impl Error for AppError {}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError{
            code: IO_ERROR,
            message: value.to_string()
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(value: num::ParseIntError) -> Self {
        AppError { code: NUMBER_PARSE_ERROR, message: value.to_string() }
    }
}

const IO_ERROR: i32 = 10000;
const NUMBER_PARSE_ERROR: i32 = 10001;

pub fn study_error() {
    let r = read_file();
    println!("result:{:?}", r);
}

fn read_file() -> Result<String, AppError> {
    let file = fs::read_to_string("/home/yaolei")?;
    Ok(file)
}

fn parse() -> Result<i32, AppError> {
    let s = "5";
    i32::from_str_radix(src, radix)
    let x = i32::from_str(s).unwrap();
    let s1 = "a123";
    let r:i32 = s1.parse()?;
    Ok(r)
}