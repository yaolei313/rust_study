mod authentication;
mod test;

use std::collections::HashMap;
use std::io;

// 基本类型
/*
 * i8       u8
 * i16      u16
 * i32      u32
 * i64      u64
 * i128     u128
 * isize    usize  isize 和 usize 类型取决于运行程序的计算机的类型。 在64位体系结构上使用64位类型，在32位上使用32位类型。 如果未指定整数的类型，并且系统无法推断类型，则默认情况下，系统会分配 i32 类型
 * f32      f64
 * bool
 * char     21位，但宽度会被填充至32位
 * &str     String  字符串字面量，不可变。字符串对象，可变。
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
    color: String,
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

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    let car_q = car_quality(miles);
    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type
    // - Print details for New or Used car based on roof type
    if car_q.0 == Age::Used && roof {
        println!(
            "Prepare a used car: {:?}, {}, Hard top, {} miles\n",
            motor, color, miles
        );
    }

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - Bind "age" to tuple returned from car_quality(miles)
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

// Build "Car" using input arguments
fn car_factory2(order: i32, miles: u32) -> Car {
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
    let mark_1 = Grades('A', 'A', 'A', 'A', 3.75);
    let u_1 = Unit {};
    println!("{:?}, {:?}, {:?}", user_1, mark_1, u_1);
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

fn study_array_and_vec() {
    // 数组长度不可变
    let array_1 = ["a", "b", "c"];
    let mut array_2: [i32; 5] = [0; 5];
    array_2[2] = 2; // 修改数据内容的话，必须增加mut
    println!("array1: {:?},array2: {:#?}", array_1, array_2);

    // 长度可变
    let vec_1 = vec![1, 2, 3];
    println!("{} {:?} {:?}", vec_1[0], vec_1.get(1), vec_1.get(99)); // 下标访问越界会panic，get不会
    let vec_2: Vec<i32> = vec![0; 5];
    let mut vec_3: Vec<char> = Vec::new();
    vec_3.push('a');
    vec_3.push('b');
    let item = vec_3.pop(); // 从vec尾部pop一个值出来
    vec_3.push('d');
    vec_3.push('e');
    vec_3[1] = 'c'; // vec可以直接修改，不需要mut标识
    println!(
        "vec_1: {:?}, vec_2: {:?}, vec_3: {:?}, {:?}",
        vec_1, vec_2, vec_3, item
    );
    let vec_4: Vec<char> = vec_3.drain(0..vec_3.len()).collect();
    println!("vec_4: {:?}", vec_4);
}

fn study_map() {
    let mut name_map: HashMap<&str, &str> = HashMap::new();
    name_map.insert("abc", "1");
    name_map.insert("Programming in Rust", "Great examples.");

    let desc = name_map.get("Programming in Rust");
    println!("desc: {:?}", desc);

    let mut map2 = HashMap::new();
    map2.insert("k", 123);
    map2.insert("k1", 234);
    map2.remove("k");
    println!("map2: {:?}", map2)
}

fn study_complex_type() {
    let mut map1 = HashMap::new();
    let mut order_id = 0;

    let colors = ["Blue", "Green", "Red", "Sliver"];
    let mut engine;
    // Order 3 cars, one car for each type of transmission

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    let mut car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order : {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
    order_id += 1;
    map1.insert(order_id, car);

    // Car order #1: New, Manual, Hard top
    engine = Transmission::Manual;
    car = car_factory(String::from("Orange"), engine, true, 0);
    order_id += 1;
    map1.insert(order_id, car);
    println!("car order {} {:?}", order_id, map1.get(&order_id));

    // Car order #2: Used, Semi-automatic, Convertible
    car = car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);
    order_id += 1;
    map1.insert(order_id, car);
    println!("car order {} {:?}", order_id, map1.get(&order_id));
}

fn build_car() {
    let mut orders = HashMap::new();
    let mut miles = 0;
    let mut car;
    for order in 1..7 {
        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map
        car = car_factory2(order, miles);
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

fn study_loop() {
    let mut count = 1;
    let stop_count = loop {
        count += 1;
        if count == 100 {
            break count;
        }
    };
    println!("break count at {}", stop_count);

    while count < 200 {
        count += 10
    }
    println!("after while loop as {}", count);

    let birds = ["ostrich", "peacock", "stork"];
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

struct Rectangle {
    width: f64,
    height: f64,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {}, height: {}", self.width, self.height)
    }
}

impl Iterator for Circle {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        todo!()
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
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
    let rectangle = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    println!("object {} {}", circle, rectangle)
}

fn study_module() {
    let mut user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    user.set_password("even-more-secret");
}

fn main() {
    println!("guess the number, {}", "yao");
    println!("please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // todo!("print this message on compile");

    study_struct();
    study_enum();
    study_array_and_vec();
    study_map();
    study_complex_type();

    study_loop();
    study_trait();
    study_module();
}
