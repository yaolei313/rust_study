use std::collections::HashMap;

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
 * tuple
 * array
 *
 * String  s[start..endExclude] slice
 * &str 实际就是String slice类型的不可边引用
 * slice类型，没有对象所有权：
 * &str,&[i32]
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
    // 数组长度不可变
    let array_1 = ["a", "b", "c"];
    let mut array_2: [i32; 5] = [0; 5];
    array_2[2] = 2; // 修改数据内容的话，必须增加mut
    println!("array1: {:?},array2: {:#?}", array_1, array_2);

    // Vec长度可变
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
    println!("map2: {:?}", map2)
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
