// package(cargo toml) -> crate -> mod
// crate根文件为src/main.rs或src/lib.rs
// 在crate根文件中声明模块后，比如mod gardon，编译器会到如下位置寻找模块的定义：
// 1.内联，mod gardon后的{}内
// 2.src/gardon.rs
// 3.src/gardon/mod.rs  (老风格，不推荐)
// 声明子模块，在非crate根文件中定义的模块，都是子模块。比如mod vegetables
// 1.内联
// 2.src/gardon/vegetables.rs
// 3.src/gardon/vegetables/mod.rs  (老风格，不推荐)

mod authentication;
mod base;
mod s_unit_test;

use std::collections::HashMap;
use std::io;

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
        color: String::from(colors[color - 1]),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
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
    base::study_option();
    base::study_enum_and_match();

    base::s_struct_trait::study_struct();

    base::study_loop();
    study_trait();
    study_module();
}
