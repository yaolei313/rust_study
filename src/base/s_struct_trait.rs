use std::{
    fmt::{Debug, Display},
    time::SystemTime,
};

/*
 * struct分为3类
 * classic struct
 * tuple struct 使用序号访问字段，从0开始
 * unit-like struct 实现trait使用，但不需要存储
 */
// Classic struct with name field
#[derive(Debug)]
struct Student {
    name: String,
    level: u8,
    remote: bool,
}

// tuple struct
#[derive(Debug)]
struct Grades(char, char, char, char, f32);

// unit struct
#[derive(Debug)]
struct Unit;

pub fn study_struct() {
    let user_1 = Student {
        name: String::from("libai"),
        level: 1,
        remote: true,
    };
    let m1 = Grades('A', 'A', 'A', 'A', 3.75);
    let u_1 = Unit {};
    println!("{:?}, {:?}, {:?}", user_1, m1.4, u_1);
}

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

pub fn study_genericity() {
    let p1 = Point { x: 10, y: 20 };
    println!("x: {}, y:{}", &p1.x, p1.y())
}

// --------------

pub trait Summary {
    fn summarize_author(&self) -> String;
    // 定义默认实现
    fn summarize(&self) -> String {
        String::from("read more...")
    }
}

pub struct Tweet {
    pub author: String,
    pub content: String,
    pub time: u64,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
    pub time: u64,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("/by {}", self.author)
    }
}

// trait作为参数
// 简化写法，实际为trait bound的语法糖
pub fn notify(s: &impl Summary) {
    println!("break news: {}", s.summarize());
}

// trait bound
pub fn notify2<T: Summary>(s: &T) {
    println!("break news: {}", s.summarize());
}

// 参数需要实现多个trait
pub fn notify3(s: &(impl Summary + Display)) {
    println!("break news: {}", s.summarize());
}

pub fn notify4<T: Summary + Display>(s: &T) {
    println!("break news: {}", s.summarize());
}

pub fn some_fn<T: Summary + Clone, U: Clone + Debug>(t: &T, u: &U) {
    todo!("empty")
}

// 通过where简化trait bound
pub fn some_fn2<T, U>(t: &T, u: &U)
where
    T: Summary + Clone,
    U: Clone + Debug,
{
    todo!("empty")
}

// trait作为返回参数
pub fn return_summary1() -> impl Summary {
    let now_unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backward")
        .as_secs();
    Tweet {
        author: String::from("libai"),
        content: String::from("123"),
        time: now_unix_timestamp,
    }
}

// trait作为返回参数 pub fn return_summary(switch: bool) -> impl Summary {}
pub fn return_summary2(switch: bool) -> Box<dyn Summary> {
    let now_unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backward")
        .as_secs();
    if switch {
        Box::new(Tweet {
            author: String::from("libai"),
            content: String::from("123"),
            time: now_unix_timestamp,
        })
    } else {
        Box::new(NewsArticle {
            headline: String::from("break news"),
            author: String::from("wang wu"),
            content: String::from("123"),
            time: now_unix_timestamp,
        })
    }
}
