use std::collections::HashMap;

use rust_study;

/// #const常量
/// 和let相比
/// 定义时必须赋值；必须指定类型;名称全部大写;定义时必须赋值且编译期就可以确定值;可以在任意作用域定义，其生命周期等同整个程序的生命周期，编译器会尽可能内敛代码。
const NAME: &str = "我没啥优点，就是活得久，嘿嘿";
/// static全局变量
/// 和const相比，不会inline；且是可变的mut；多线程访问的话，不安全；必须实现Sync trait；定义时必须赋值且编译期就可以确定值；名称也是必须大写。
static NAME2: &str = "hello world";

/// #基本类型
///
/// integer type:
/// i8       u8            u8字面量b'A'
/// i16      u16
/// i32      u32
/// i64      u64
/// i128     u128
/// isize    usize  isize 和 usize 类型取决于运行程序的计算机的类型。 在64位体系结构上使用64位类型，在32位上使用32位类型。 如果未指定整数的类型，并且系统无法推断类型，则默认情况下，系统会分配 i32 类型
///
/// floating-point type:
/// f32      f64
///
/// boolean type:
/// bool
///
/// character type:
/// char     使用单引号，21位，但宽度会被填充至32位, 4字节代表一个unicode字符,U+0000,U+D7FF,U+E000~U+10FFFF
///
/// compound type:
/// tuple    ()
/// array    []
///
/// collection type:
/// Vec<>
/// String  堆上分配内存，s[start..endExclude] to slice
/// HashMap<>
///
/// slice type:(slice类型，没有对象所有权)
/// &str     字符串字面量，不可变。字符串对象，可变。
/// &[i32]   i32类型array的引用
///
/// struct,enum
///
/// == 等于 PartialEq，Eq 2个trait，前者不满足自反性，即x==x，仅满足对称性x==y则y==x和传递性x==y,y==z,则x==z
///
/// package包含Cargo.toml，描述了如何build多个crate
/// crate分为2种，binary crate和library crate，前者包含main入口，可以编译为executable file
///
/// rust编译的最小代码单元是crate，crate root是一个source file。分别为src/lib.rs,src/main.rs
////
pub fn study_primative_type() {
    let num: u32 = "313".parse().expect("not a number");
    let r1 = num.wrapping_add(10);
    let r2 = num.checked_add(12);
    println!("result: {} {}", r1, r2.unwrap());

    let s1 = String::from("abc");
    s1.len();
}

pub fn study_compound_type() {
    println!("====================study_array_and_vec");

    // tuple
    let t3 = (1, 2, 3);
    println!("tuple is {} {} {}", t3.0, t3.1, t3.2);

    // array，数组长度不可变
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
}

pub fn study_collection_type() {
    study_string();
    study_vec();
    study_map();
    study_slice();
}

fn study_string() {
    let s1: String = String::from("hello world");
    let s2 = rust_study::first_world(&s1);
    println!("string is {}", s2);

    // 参数为&mut self,即会同时存在mut borrow和immutable borrow
    // s1.clear();

    let s3 = "a";
    let s4 = "Зд";

    let s5 = format!("{s3}-{s4}");
    for c in s5.chars() {
        print!("{c}\t");
    }
}

fn study_vec() {
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

    // retain：修改自身，保留符合条件的
    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    v1.retain(|x| x % 2 == 0);

    // drain: 修改自身，删除符合条件的
    let mut v2 = vec![1, 2, 3, 4, 5, 6];
    let v3: Vec<i32> = v2.drain(2..).collect();
    assert_eq!(v2, &[1, 2]);
    assert_eq!(v3, &[3, 4, 5, 6]);
    println!("v2 {:?} v3 {:?}", v2, v3);
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

fn study_slice() {
    // slice类型，没有对象所有权

    // 数组slice
    let a1 = [2, 4, 5, 8, 10];
    let b1: &[i32] = &a1[0..3];
    println!("array slice is {:?}", b1);

    let s1 = "hello world";
    let s2 = &s1[0..5];
    println!("str slice is {}", s2);

    let mut v1 = vec![1, 2, 3, 4];
    let v1_slice: &[i32] = &v1;
    // v1.push(5);
    println!("{:?}", v1_slice);

    let v1_slice2 = &mut v1[..];
    println!("{:?}", v1_slice2);
}
