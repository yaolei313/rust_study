use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

pub fn study_option() {
    // Ending the expression with [`?`] will result in the [`Some`]'s unwrapped value, unless the
    // result is [`None`], in which case [`None`] is returned early from the enclosing function.
    //
    //[`?`] can be used in functions that return [`Option`] because of the
    //early return of [`None`] that it provides.
    let mut stack = vec![1, 2, 3];
    let result = add_last_numbers(&mut stack);
    println!("result is : {}", result.unwrap_or_default());

    // match 匹配option
    match result {
        None => println!("result is None"),
        Some(v) => println!("result is {}", v),
    }

    let b: Option<&str> = None;
    let b2 = b.unwrap_or("hello");
    println!("b2 {}", b2);

    let mut c = Some(10);
    c.take();
}

fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    // ?可以用到返回Option类型的语句之后，若返回值是None，会立即return None，若返回值是Some，则执行unwrapped
    Some(stack.pop()? + stack.pop()?)
}

pub fn study_result() {
    let greeting_file_result = File::open("target/hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("target/hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("create file failed {:?}", e),
            },
            other => panic!("open file failed {:?}", other),
        },
    };

    // Result<T,E> unwrap若为Err则返回panic，expect相比可以指定错误message
    let greeting_file2 = File::open("target/hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("target/hello2.txt").unwrap_or_else(|error| panic!("create file failed {:?}", error))
        } else {
            panic!("open file failed {:?}", error)
        }
    });
}

/// 错误传播?操作符
/// ?可以用到返回Option类型的语句之后，若返回值是None，会立即return None，若返回值是Some，则执行unwrap
/// ?可以用到返回Result类型的语句之后，若返回只是Err，则return Err，若返回值是Ok，则执行unwrap获取包装的值
/// 也可以自动调用目标Error的From trait实现，完成error类型的装换。anyhow::Result
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, Box<dyn std::error::Error>> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn err_convert() -> Result<File, Box<dyn Error>> {
    let r = File::open("1.txt");
    let f = match r {
        Ok(f) => f,
        Err(e) => {
            let n = std::convert::From::from(e);
            return Err(n);
        }
    };
    Ok(f)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[cfg(test)]
mod test {
    //use core::slice::memchr;
    use anyhow::Context;
    use encoding_rs_io::DecodeReaderBytesBuilder;
    use std::fs::File;
    use std::io::{BufRead, BufReader, ErrorKind, Read};

    #[test]
    fn test1() -> anyhow::Result<()> {
        let file = File::open("/Users/yao/Downloads/test1.txt").with_context(|| "file not found")?;
        let decoded_reader = DecodeReaderBytesBuilder::new().build(file);
        let mut reader = BufReader::new(decoded_reader);
        let mut buf = Vec::with_capacity(10 * 1024);
        let mut counter = 0;
        loop {
            match reader.read_until(b'\n', &mut buf) {
                Ok(i) => {
                    if i == 0 {
                        break;
                    }
                    counter += 1;
                    let line = String::from_utf8(buf[..i].to_vec())?;
                    println!("{} {}", counter, line);
                    buf.clear();
                }
                Err(e) => {
                    return Err(e.try_into()?);
                }
            }
        }
        Ok(())
    }

    // fn read_until<R: BufRead + ?Sized>(r: &mut R, delim: &[u8; 2], buf: &mut Vec<u8>) -> std::io::Result<usize> {
    //     let mut read = 0;
    //     loop {
    //         let (done, used) = {
    //             let available = match r.fill_buf() {
    //                 Ok(n) => n,
    //                 Err(e) => return Err(e),
    //             };
    //             match memchr::memchr(delim[0], available) {
    //                 Some(i) => {
    //                     if i + 1 < available.len() && available[i + 1] == delim[1] {
    //                         buf.extend_from_slice(&available[..=i + 1]);
    //                         (true, i + 2)
    //                     } else {
    //                         buf.extend_from_slice(available);
    //                         (false, available.len())
    //                     }
    //                 }
    //                 None => {
    //                     buf.extend_from_slice(available);
    //                     (false, available.len())
    //                 }
    //             }
    //         };
    //         r.consume(used);
    //         read += used;
    //         if done || used == 0 {
    //             return Ok(read);
    //         }
    //     }
    // }
}
