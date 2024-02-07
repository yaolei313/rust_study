use ahash::{AHasher, RandomState};
use std::{collections::HashMap, fmt::Display};

use basic_utils;

use crate::s_pattern_match::Point;

/// #const常量
/// 和let相比
/// 定义时必须赋值；必须指定类型;名称全部大写;定义时必须赋值且编译期就可以确定值;可以在任意作用域定义，其生命周期等同整个程序的生命周期，编译器会尽可能内联代码。
const NAME: &str = "我没啥优点，就是活得久，嘿嘿";
/// static全局变量
/// 和const相比，不会inline；且可以是可变的mut；多线程访问的话，不安全；必须实现Sync trait；定义时必须赋值且编译期就可以确定值；名称也是必须大写。
static NAME2: &str = "hello world";

///
/// #基本类型，所有基本类型都实现了copy trait，会自动被copy
///
/// integer type:
/// i8       u8            u8字面量b'A'
/// i16      u16
/// i32      u32           整数默认类型i32
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
/// tuple    ()  仅包含实现了copy trait的tuple，array也是可以copy的类型
/// array    []  [std::ops::Index] trait
///
/// collection type:
/// Vec<>
/// String  堆上分配内存，s[start..endExclude] to slice
/// HashMap<>
/// Range   序列只允许数字或者字符类型，因为它们是连续的，比如 0..4 范围不包括4,  0..=4 范围包括4
///
/// slice type:(slice类型，没有对象所有权，也是可以copy的类型)  
/// &str     字符串字面量，不可变。字符串对象，可变。
/// &[i32]   i32类型array的引用
///
/// * 裸指针raw pointer
///     *const T / *mut T 分别代表可变和不可变的裸指针，不可变意味着解引用后不能直接赋值，需要使用unsafe关键字，栈上空间为usize
/// * 引用reference
///     &T / &mut T 内存对齐的指向T的有效的值指针，表示对该T元素的借用，栈上空间为usize
/// * Array
///     [T; N],栈上空间为Sizeof(T)*N
/// * String
///     std::String，栈上空间为3*usize，ptr，len，cap
/// * Slice
///     str [T] 所有切片类型都是unsize type。unsizetype除此外还包括dyn trait，无法直接使用，必须通过引用和Box<>来使用  
/// * Slice Reference
///     &[T] / &mut [T]，栈上空间为2*usize，ptr，len
/// * String slice
///     &str，utf8编码的字符串，栈上空间为2*usize
/// * Box
///     std::Box<T, A = Global>，指向堆上数据的指针，栈上空间为usize，ptr
/// * Vec
///     std::Vec<T, A = Global>，栈上空间为3*usize，ptr，len，cap
///
///
/// == 等于 PartialEq，Eq 2个trait，前者不满足自反性，即x==x，仅满足对称性x==y则y==x和传递性x==y,y==z,则x==z
///
/// package包含Cargo.toml，描述了如何build多个crate
/// crate分为2种，binary crate和library crate，前者包含main入口，可以编译为executable file
///
/// rust编译的最小代码单元是crate，crate root是一个source file。分别为src/lib.rs,src/main.rs
////
pub fn study_primative_type() {
    // 前者使用显示的类型标注，后者直接制定泛型类型
    let num: i8 = "127".parse().expect("not a number");
    let num2 = "312".parse::<i32>().expect("not a number");
    println!("num {num} {num2}");
    // wrapping_* 补码循环溢出
    // checked_* 发生溢出，返回NONE值
    // overflowing_* 返回值和一个bool值指示是否溢出
    // saturating_* 值只能到最大值或者最小值
    let r1 = num.wrapping_add(10);
    let r2 = num.checked_add(10);
    let (r3, over) = num.overflowing_add(10);
    let r4 = num.saturating_add(10);
    println!("result: {} {} {} {} {}", r1, r2.unwrap(), r3, over, r4);

    let n1 = 10 / 3;
    println!("div result: {}", n1);

    println!("The size of raw pointer: {}", std::mem::size_of::<*const u64>()); // 8  bytes
    println!("The size of reference: {}", std::mem::size_of::<&u64>()); // 8  bytes
    println!("The size of slice: {}", std::mem::size_of::<&[u8]>()); // 16 bytes
    println!("The size of box: {}", std::mem::size_of::<Box<u8>>()); // 8  bytes
    println!("The size of box slice: {}", std::mem::size_of::<Box<[u8]>>()); // 16 bytes
    println!("The size of vec: {}", std::mem::size_of::<Vec<u8>>()); // 24 bytes
    println!("The size of String: {}", std::mem::size_of::<String>()); // 24 bytes
    println!("The size of string slice: {}", std::mem::size_of::<&str>()); // 16 bytes
    println!("The size of Array[u8;3]: {}", std::mem::size_of::<[u8; 3]>()); // 3 bytes

    //
    let s1 = "123";
    let p = s1.as_ptr();

    let t1 = b"hello world";
    let b1 = t1.starts_with(b"hello");
}

pub fn study_compound_type() {
    println!("====================study_array_and_vec");

    // tuple
    let t3 = (1, 2, 3);
    println!("tuple is {} {} {}", t3.0, t3.1, t3.2);

    // array，数组长度不可变。可以通过[1,1,1]或[1;3]创建
    let array_1 = ["a", "b", "c"];

    let obj_array = [Point { x: 1, y: 2 }];

    // 访问array的下标是usize类型，需要使用as做类型转换
    let number: i32 = 1;
    let idx: usize = number as usize;
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
    let s2 = basic_utils::first_world(&s1);
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

    let mut v4 = [11, 22, 33].to_vec();
    let v5 = v4.split_off(1);
}

fn study_map() {
    let mut name_map: HashMap<&str, &str> = HashMap::new();
    name_map.insert("abc", "1");
    name_map.insert("Programming in Rust", "Great examples.");

    for (k, v) in &name_map {
        println!("{k}: {v}");
    }

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

    // rust默认使用安全hash算法，性能会差一些
    let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
    map.insert(1, 123);
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
    let s4_str1 = &s4[..];

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

    study_enum_convert();
}

pub fn study_enum_convert() {
    let v = TestEnum::A as i32;
    println!("enum to int {}", v);

    let e: TestEnum = v.try_into().expect("convert fail");
    // 可以通过泛型为所有类型实现某个trait。TryInto trait就是。相当于如下代码。
    let e2 = TestEnum::try_from(v).expect("convert fail");
    println!("int to enum {0} {0:?} {1:?}", e, e2);
}

#[derive(Debug)]
enum TestEnum {
    A,
    B,
    C,
}

impl TryFrom<i32> for TestEnum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == TestEnum::A as i32 => Ok(TestEnum::A),
            y if y == TestEnum::B as i32 => Ok(TestEnum::B),
            z if z == TestEnum::C as i32 => Ok(TestEnum::C),
            _ => Err(()),
        }
    }
}

impl Display for TestEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestEnum::A => write!(f, "TestEnum-A"),
            TestEnum::B => write!(f, "TestEnum-B"),
            TestEnum::C => write!(f, "TestEnum-C"),
        }
    }
}
