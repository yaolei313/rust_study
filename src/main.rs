mod authentication;
mod base;
mod test;

use std::collections::HashMap;
use std::io;

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

#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEKey(KeyPress),
    WEClick(MouseClick),
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Declare Car struct to describe vehicle with four named fields
#[derive(PartialEq, Debug)]
struct Car {
    // 使用String类型而不是&str是为了拥有所有权，若使用&str，则需要lifecycle标识符
    color: String,
    // name: &str,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality(miles: u32) -> (Age, u32) {
    // if可以是表达式，所有分支必须返回相同类型
    // let quality: (Age, u32) =
    if miles == 0 {
        (Age::New, miles)
    } else {
        (Age::Used, miles)
    }
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    // 访问array的下标是usize类型
    let mut color = order as usize;
    while color > 4 {
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof: bool = true;
    if order % 3 == 0 {
        // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    } // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color - 1)]),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn study_struct() {
    let user_1 = Student {
        name: String::from("libai"),
        level: 1,
        remote: true,
    };
    let m1 = Grades('A', 'A', 'A', 'A', 3.75);
    let u_1 = Unit {};
    println!("{:?}, {:?}, {:?}", user_1, m1.4, u_1);
}

fn study_enum() {
    // algebraic data type
    let we_load = WebEvent::WELoad(true);

    let m_click = MouseClick { x: 100, y: 80 };
    let we_click = WebEvent::WEClick(m_click);

    let t_keypress = KeyPress(String::from("Ctrl+"), 'N');
    let we_key = WebEvent::WEKey(t_keypress);

    // 通过 #[derive(Debug)] 语法可以在代码执行期间查看某些在标准输出中无法查看的值。 要使用 println! 宏查看调试数据，请使用语法 {:#?} 以可读的方式格式化数据。
    println!(
        "\nWebEvent enum structure: \n\n {:?} \n\n {:?} \n\n {:#?}",
        we_load, we_click, we_key
    );

    // match模式匹配
    match we_load {
        WebEvent::WELoad(a) => {
            println!("a {}", a);
        }
        _ => {
            println!("other")
        }
    }

    let a_number = Some(8);
    // if let表达式，单个模式匹配
    if let Some(v) = a_number {
        println!("value is {}", v);
    }
    assert_eq!(a_number.unwrap(), 8);
    //assert_eq!(a_number.expect("not match custom panic"), 9)

    let b: Option<&str> = None;
    let b2 = b.unwrap_or("hello");
    println!("b2 {}", b2);
}

fn build_car() {
    let mut orders = HashMap::new();
    let mut miles = 0;
    let mut car;
    for order in 1..7 {
        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "radius: {}", self.radius)
    }
}

impl Area for Circle {
    // &self实际是self: &Seft的缩写，在impl代码块中，Self是类型的别名，此处Self代表Circle
    // 方法可以获取self的所有权，也可以不可变的借用&self，也可以可变的借用&mut self
    // 仅使用self很少见，一般场景为self转换为别的实例的时候，防止调用方使用转换前的视力
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Iterator for Circle {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        todo!()
    }
}

fn convert_to_i32(default: i32, str: &str) -> i32 {
    let r = str.trim().parse::<i32>();
    if r.is_err() {
        default
    } else {
        r.unwrap()
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(s: &str) -> Rectangle {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        if s.len() != 0 {
            let v: Vec<&str> = s.split(',').collect();
            if v.len() == 2 {
                if let Some(s) = v.get(0) {
                    width = convert_to_i32(0, s);
                }
                if let Some(s) = v.get(1) {
                    height = convert_to_i32(0, s);
                }
            }
        }
        Rectangle { width, height }
    }

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

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

fn study_trait() {
    let circle = Circle { radius: 10.0 };
    let r1 = Rectangle::new("10,11");
    dbg!(&r1);
    // struct update syntax
    let r = Rectangle { width: 5, ..r1 };

    // 自动引用和解引用。
    // 使用object.something()调用方法时，Rust会自动为object添加 &、&mut 或 * 以便使object与方法签名匹配。
    // 即r.area()实际等同(&r).area();
    println!("area is:{} {}", r.area(), (&r).area());
    println!("test dgb!");
    dbg!(&r);
    println!("object {} {}", circle, r);
}

fn study_module() {
    let mut user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    user.set_password("even-more-secret");
}

fn main() {
    let age = 30;
    println!("guess the number, {} {age}", "yao");
    println!("please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // todo!("print this message on compile");
    base::study_primative_type();
    base::study_array_and_vec();
    base::study_map();
    base::study_slice();

    study_struct();
    study_enum();

    base::study_loop();
    study_trait();
    study_module();
}
