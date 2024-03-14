use std::{error, num};
use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::{self, Read},
};

#[derive(Debug)]
pub struct AppError {
    code: i32,
    message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, message: {}", self.code, self.message)
    }
}

impl Error for AppError {}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError {
            code: IO_ERROR,
            message: value.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(value: num::ParseIntError) -> Self {
        AppError {
            code: NUMBER_PARSE_ERROR,
            message: value.to_string(),
        }
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
    let x = i32::from_str_radix(s, 10).unwrap();
    let s1 = "a123";
    // 通过std::num中的宏实现from_str_radix_int_impl
    // macro_rules! from_str_radix_int_impl {
    //     ($($t:ty)*) => {$(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl FromStr for $t {
    //             type Err = ParseIntError;
    //             fn from_str(src: &str) -> Result<Self, ParseIntError> {
    //                 from_str_radix(src, 10)
    //             }
    //         }
    //     )*}
    // }
    // from_str_radix_int_impl! { isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128 }
    let r: i32 = s1.parse()?;
    Ok(r)
}

/// https://github.com/dtolnay/thiserror
#[derive(thiserror::Error, Debug)]
enum MyError {
    // #[error] 代表实现Display trait,#[from] 代表实现From trait
    #[error("custom io error")]
    IoError(#[from] io::Error),
    #[error("the param {0} is error")] // write!(f, "{}", self.0)
    ParamError(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    // write!(f, "{:?} {:?}", self.expected, self, found)
    InvalidHeader { expected: String, found: String },
    #[error("test")]
    NumberParseError(#[from] num::ParseIntError),
    #[error("unknow error")]
    Unknown,
}

fn test() -> anyhow::Result<String> {
    todo!()
}
