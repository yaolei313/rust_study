use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

pub mod s_closure;
pub mod s_data_type;
pub mod s_genericity;
pub mod s_iter;
pub mod s_lifetime;
pub mod s_smart_pointer;
pub mod s_struct_trait;
pub mod s_thread;

pub fn study_type_convert() {
    // Array 转 slice &[i32]
    let a = [72, 101, 108, 108, 111];
    let a_slice: &[i32] = &a;
    let b_slice = &a[..];

    // Vec<u8> 转 slice &[u8], 只需要一个&符号?
    let v: Vec<u8> = vec![72, 101, 108, 108, 111]; // "Hello"
    let v_slice: &[u8] = &v;

    // &[u8] 转 Vec<u8>
    let v_slice: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    let v_vec: Vec<u8> = v_slice.to_vec(); // copy

    println!("---------------");

    // s: &str -> String, String::from(s)   s.to_string()    s.to_owned()
    let s1 = "abc";
    let s1_string = String::from(s1);

    // s: &str -> &[u8],   s.as_bytes()
    let s2 = "cde";
    let s2_bytes = s2.as_bytes(); // copy

    // s: &str -> Vec<u8> s.as_bytes().to_vec()
    let s3 = "123";
    let s3_vec = s3.as_bytes().to_vec(); // to_vec copy

    // s: String -> &str
    let s4 = String::from("123");
    let s4_str = s4.as_str();

    // s: String -> &[u8]
    let s5 = String::from("135");
    let s5_bytes = s5.as_bytes();

    // s: String -> Vec<u8>
    let s5 = String::from("123");
    let s5_vec = s5.into_bytes(); // moved, s5不能使用
                                  // println!("String -> Vec<u8>: {}", s5);

    // s: Vec<u8> -> String
    let s6 = [65, 66, 67].to_vec();
    let s6_string = String::from_utf8(s6).unwrap();
    println!("s6: {}", s6_string);

    // s: Vec<u8> -> &[u8] -> &str
    let s7: Vec<u8> = vec![68, 69, 70];
    let s7_str = std::str::from_utf8(&s7).unwrap();
}

pub fn study_condition_expression() {
    // if可以是表达式，所有分支必须返回相同类型
    let miles = 100;
    let car_desc = if miles > 0 { "new car" } else { "used car" };
    println!("{}", car_desc);
}

pub fn study_loop() {
    let mut count = 1;
    // loop
    let stop_count = loop {
        count += 1;
        if count == 100 {
            break count;
        }
    };
    println!("break count at {}", stop_count);

    // while condition {}
    while count < 200 {
        count += 10
    }
    println!("after while loop as {}", count);

    let birds = ["ostrich", "peacock", "stork"];
    // for item in xx {}
    for item in birds {
        print!("{}, \t", item);
    }
    println!("");
    for item in birds.iter() {
        print!("{}, \t", item);
    }
    println!("");
    for number in 0..5 {
        print!("{}, ", number)
    }
    println!("")
}

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
}

fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    // ?可以用到返回Option类型的语句之后，若返回值是None，会立即return None，若返回值是Some，则执行unwrapped
    Some(stack.pop()? + stack.pop()?)
}

#[derive(Debug)]
enum WebEvent {
    Key(String, char),
    Click(i64, i64),
}

pub fn study_enum_and_match() {
    // algebraic data type
    let we_click = WebEvent::Click(100, 80);
    let we_key = WebEvent::Key(String::from("Ctrl+"), 'N');

    // 若实现了std::fmt::Debug trait, 即#[derive(Debug)]，可以通过{:?}打印，{:#?}以更可读的方式格式化
    // 若实现了std::fmt::Display trait, 可以通过{}打印。
    println!(
        "\nWebEvent enum structure: \n\n {:?} \n\n {:#?}",
        we_click, we_key
    );

    // match模式中的other和_,other[任意名称]会匹配任意值并绑定值，_匹配任意值但忽略绑定值
    let roll = 9;
    let mut t1 = String::from("e");
    let r2: &str = match roll {
        1 => "a",
        2 => "b",
        other => {
            let t2 = other.to_string();
            t1.push_str(&t2);
            t1.as_str()
        }
    };
    println!("r2 is {}", r2);

    match roll {
        3 => roll1(),
        5 => roll2(),
        _ => (),
    }

    // match模式匹配
    match we_click {
        WebEvent::Click(a, b) => {
            println!("a {} b {}", a, b);
        }
        _ => {
            println!("other")
        }
    }

    let a_number = Some(8);
    // if let表达式，单个模式匹配
    if let Some(v) = a_number {
        println!("value is {}", v);
    } else {
        println!("other")
    }
    assert_eq!(a_number.unwrap(), 8);
    //assert_eq!(a_number.expect("not match custom panic"), 9)
}

fn roll1() {}
fn roll2() {}

pub fn study_result() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("create file failed {:?}", e),
            },
            other => panic!("open file failed {:?}", other),
        },
    };

    // Result<T,E> unwrap若为Err则返回panic，expect相比可以指定错误message
    let greeting_file2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt")
                .unwrap_or_else(|error| panic!("create file failed {:?}", error))
        } else {
            panic!("open file failed {:?}", error)
        }
    });
}

// 错误传播?操作符
// ?可以用到返回Option类型的语句之后，若返回值是None，会立即return None，若返回值是Some，则执行unwrapped
// ?可以用到返回Result类型的语句之后，若返回只是Err，则return Err，若返回值是Ok，会返回Ok包装的值
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn largest<T: std::cmp::PartialOrd>(input: &[T]) -> &T {
    let mut largest = &input[0];
    for i in input {
        if i > largest {
            largest = i;
        }
    }
    return largest;
}
