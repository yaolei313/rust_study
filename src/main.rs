use std::collections::HashMap;
use std::io;

// 基本类型
/*
 * i8       u8
 * i16      u16
 * i32      u32
 * i64      u64
 * i128     u128
 * isize    usize
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
#[derive(Debug)]
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
}

fn study_array_and_vec() {
    // 数组长度不可变
    let array_1 = ["a", "b", "c"];
    let mut array_2: [i32; 5] = [0; 5];
    array_2[2] = 2;
    println!("array1: {:?},array2: {:#?}", array_1, array_2);

    // 长度可变
    let vec_1 = vec![1, 2, 3];
    let vec_2: Vec<i32> = vec![0; 5];
    let mut vec_3 = Vec::new();
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
}

fn study_map() {
    let mut name_map: HashMap<&str, &str> = HashMap::new();
    name_map.insert("abc", "1");
    name_map.insert("Programming in Rust", "Great examples.");

    let desc = name_map.get("Programming in Rust");
    println!("desc: {:?}", desc);
}

fn study_complex_type() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false, 0);
    println!(
        "Car 1 = {}, {:?} transmission, roof: {}, mileage: {:?}",
        car.color, car.motor, car.roof, car.age
    );

    let colors = ["Blue", "Green", "Red", "Sliver"];
    let mut engine = Transmission::Manual;
    // Order 3 cars, one car for each type of transmission

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
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
}
