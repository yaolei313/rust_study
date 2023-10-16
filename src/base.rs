use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

pub mod s_closure;
pub mod s_genericity;
pub mod s_lifetime;
pub mod s_struct_trait;

// 基本类型
/*
 * i8       u8            u8字面量b'A'
 * i16      u16
 * i32      u32
 * i64      u64
 * i128     u128
 * isize    usize  isize 和 usize 类型取决于运行程序的计算机的类型。 在64位体系结构上使用64位类型，在32位上使用32位类型。 如果未指定整数的类型，并且系统无法推断类型，则默认情况下，系统会分配 i32 类型
 * f32      f64
 * bool
 * char     21位，但宽度会被填充至32位, 4字节代表一个unicode字符,U+0000,U+D7FF,U+E000~U+10FFFF
 * &str     String  字符串字面量，不可变。字符串对象，可变。
 * tuple    ()
 * array    []
 *
 * String  s[start..endExclude] slice
 * &str 实际就是String slice类型的不可变引用
 * &[i32] i32类型array的引用
 * slice类型，没有对象所有权
 *
 * Vec<>,HashMap<>类型都可能涉及到所有权的转移
 *
 * == 等于 PartialEq，Eq 2个trait，前者不满足自反性，即x==x，仅满足对称性x==y则y==x和传递性x==y,y==z,则x==z
 *
 * package包含Cargo.toml，描述了如何build多个crate
 * crate分为2种，binary crate和library crate，前者包含main入口，可以编译为executable file
 *
 * rust编译的最小代码单元是crate，crate root是一个source file。分别为src/lib.rs,src/main.rs
 */
pub fn study_primative_type() {
    let num: u32 = "313".parse().expect("not a number");
    let r1 = num.wrapping_add(10);
    let r2 = num.checked_add(12);
    println!("result: {} {}", r1, r2.unwrap());

    let s1 = String::from("hello world");
    let s2 = first_world(&s1);
    println!("string is {}", s2);

    // 参数为&mut self,即会同时存在mut borrow和immutable borrow
    // s1.clear();

    // tuple
    let t3 = (1, 2, 3);
    println!("tuple is {} {} {}", t3.0, t3.1, t3.2);

    // array
    let a1 = [2, 4, 5];
    println!("array is {:?}", a1);

    let s3 = "a";
    let s4 = "Зд";

    let s5 = format!("{s3}-{s4}");
    for c in s5.chars() {
        print!("{c}\t");
    }
    println!();
}

fn first_world(s: &str) -> &str {
    let bs = s.as_bytes();
    for (i, &item) in bs.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

pub fn study_array_and_vec() {
    println!("====================study_array_and_vec");
    // 数组长度不可变
    let array_1 = ["a", "b", "c"];

    // 访问array的下标是usize类型，需要使用as做类型转换
    let number: i32 = 1;
    let idx = number as usize;
    println!("array idx need usize type {}", array_1[idx]);

    let mut array_2: [i32; 5] = [0; 5];
    array_2[2] = 2; // 修改数据内容的话，必须增加mut
    println!("array1: {:?},array2: {:#?}", array_1, array_2);

    // 遍历
    for i in array_2 {
        print!("item: {}", i);
    }
    println!();

    // Vec长度可变
    // vec!宏
    let mut vec_1 = vec![1, 2, 3];
    // 可以通过[i]或者get(i)读取vec中的元素，下标访问越界会panic，get不会而是返回None
    println!(
        "{} {:?} {:?}, vec_1: {:?}",
        vec_1[0],
        vec_1.get(1),
        vec_1.get(99),
        vec_1
    );
    vec_1.push(4);
    vec_1.pop();

    // 遍历并修改
    for i in &mut vec_1 {
        // 此处需要增加*解引用
        *i += 10;
    }

    // 遍历,不带&则会被moved
    for i in &vec_1 {
        print!("item: {}", i);
    }
    println!();

    // 引用的作用域是从声明开始的地方，到最后一次使用的地方。
    let t1: Option<&i32> = vec_1.get(1);
    println!("t1 {:?}", t1);

    let vec_2: Vec<i32> = vec![0; 5];
    println!("vec_2: {:?}", vec_2);

    let mut vec_3: Vec<char> = Vec::new();
    vec_3.push('a');
    vec_3.push('b');
    let item = vec_3.pop(); // 从vec尾部pop一个值出来
    vec_3.push('d');
    vec_3.push('e');
    vec_3[1] = 'c';
    println!("vec_3: {:?}, {:?}", vec_3, item);

    let vec_4: Vec<char> = vec_3.drain(0..vec_3.len()).collect();
    println!("vec_4: {:?}", vec_4);
}

pub fn study_map() {
    let mut name_map: HashMap<&str, &str> = HashMap::new();
    name_map.insert("abc", "1");
    name_map.insert("Programming in Rust", "Great examples.");

    let desc = name_map.get("Programming in Rust");
    println!("desc: {:?}", desc);

    let mut map2 = HashMap::new();
    map2.insert("k", 123);
    map2.insert("k1", 234);
    map2.remove("k");
    println!("map2: {:?}", map2);

    // 当k2不存在的时候才插入
    map2.entry("k2").or_insert(456);

    let mut count_map = HashMap::new();
    let text = "hello world wonderful world";
    for i in text.split_whitespace() {
        let count = count_map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("count map {:?}", count_map);
}

pub fn study_slice() {
    // slice类型，没有对象所有权

    // 数组slice
    let a1 = [2, 4, 5, 8, 10];
    let b1: &[i32] = &a1[0..3];
    println!("array slice is {:?}", b1);

    let s1 = "hello world";
    let s2 = &s1[0..5];
    println!("str slice is {}", s2);
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

    // 通过 #[derive(Debug)] 语法可以在代码执行期间查看某些在标准输出中无法查看的值。 要使用 println! 宏查看调试数据，请使用语法 {:#?} 以可读的方式格式化数据。
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
