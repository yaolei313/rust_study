///
/// impl代码块中定义的函数
/// * 第一个参数是self, 则叫method，通过.调用
/// * 第一个参数不是self，则叫associated function，通过::调用
///
/// 另外negative impls 可以保证对应type不会实现对应的trait。impl !XxxTrait for T
/// https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html?highlight=!trait#negative_impls
///
/// trait Xxx: Yyy 实现特征Xxx时，必须也实现Yyy。supertrait
///
/// 不能为外部struct实现外部trait，这个称为coherence 或 orphan rule规则
/// 可以通过newtype来包装某个类型，从而绕过上边的限制
///
/// &dyn XxxTrait 或 Box<dyn XxxTrait>
///
/// 不是所有的trait都可以有trait object / dyn compatible，只有对象安全的才有。即有如下限制
/// a trait is object safe if it has following qualities:
/// 1. 所有的super trait必须是object safe，[Sized]不能是super trait
/// 2. 不能包含associated constants，不能包含 associated types with generics
/// 3. 所有的associated functions,必须
/// 1. trait的所有方法(method即包含self的function)不能返回Self
/// 2. trait的所有方法没有任何泛型参数
/// 限制1，一旦使用特征对象，若trait定义的方法返回self，此时f编译器就不知道self的具体类型了。
/// 限制2未搞懂，泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。
///      而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。
/// trait GenericTrait {
///    fn add<T>(self, a: T, b: T);  
/// }
///
/// 完全限定语法
/// trait Test {
///     fn test();
/// }
///
/// impl Study {
///     fn test() {
///     }
/// }
///
/// impl Trait for Study {
///     fn test() {
///     }
/// }
///
/// <Type as Trait>::function(receiver if method, args)
///
/// <Study as Trait>::test() 调用后者
/// Study::test() 调用前者
///
/// 当使用trait特型方法时需要确保其本身必须存在当前作用域中，否则trait特型所有的方法都是不可用的；
/// 若是trait特型是[std::prelude]的一部分，这样rust就会默认自动将其引入
///
///
use std::{
    fmt::{Debug, Display},
    time::SystemTime,
};

/*
 * struct分为3类
 * classic struct
 * tuple struct 使用序号访问字段，从0开始
 * unit-like struct 实现trait使用，但不需要存储
 *
 * 若在结构体中使用引用类型，必须增加生命周期
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

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

/// 若实现了std::fmt::Debug trait, 即#[derive(Debug)]，可以通过{:?}打印，{:#?}以更可读的方式格式化
/// 若实现了std::fmt::Display trait, 可以通过{}打印。
impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "radius: {}", self.radius)
    }
}

impl Area for Circle {
    // &self实际是self: &Seft的缩写，在impl代码块中，Self是类型的别名，此处Self代表Circle
    // 方法可以获取self的所有权，也可以不可变的借用&self，也可以可变的借用&mut self。即 &self , &mut self, self
    // 仅使用self很少见，一般场景为self转换为别的实例的时候，防止调用方使用转换前的实例
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    // associated function, 没有self参数
    fn new(s: &str) -> Rectangle {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        if s.len() != 0 {
            let v: Vec<&str> = s.split(',').collect();
            if v.len() == 2 {
                if let Some(s) = v.get(0) {
                    width = basic_utils::convert_to_i32(0, s);
                }
                if let Some(s) = v.get(1) {
                    height = basic_utils::convert_to_i32(0, s);
                }
            }
        }
        Rectangle { width, height }
    }

    // method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {}, height: {}", self.width, self.height)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        (self.height * self.width) as f64
    }
}

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

pub fn study_trait() {
    let circle = Circle { radius: 10.0 };
    let r1 = Rectangle::new("10,11");
    dbg!(&r1);
    // struct update syntax, 移动了数据，r1不能再使用
    let r = Rectangle { width: 5, ..r1 };

    // 自动引用和解引用。
    // 使用object.something()调用方法时，Rust会自动为object添加 &、&mut 或 * 以便使object与方法签名匹配。
    // 即r.area()实际等同(&r).area();
    println!("area is:{} {}", r.area(), (&r).area());
    println!("test dgb!");
    dbg!(&r);
    println!("object {} {}", circle, r);
}

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
pub fn some_fn2<T, U>(_t: &T, _u: &U)
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

// 泛型可以在一个struct上实现多次，比如impl MyIterator2<i8> for Rectangle, imple MyIterator2<i32> for Rectangle等
// 但是关联类型只能实现一次，比如impl MyIterator for Rectangle
pub trait MyIterator {
    // 定义trait的associated type, 关联类型和泛型的区别是？
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait MyIterator2<T> {
    fn next(&mut self) -> Option<T>;
}

enum CarType {
    New,
    Used { age: i32 },
}

// https://santiagopastorino.com/2022/10/20/what-rpits-rpitits-and-afits-and-their-relationship/
// RPIT  Return Position Impl Trait
// RPITIT Return position `impl Trait` in traits
// AFIT Async Fn In Trait 一个async function的返回类型被脱糖为impl Future<Output = return_type_of_the_fn>
