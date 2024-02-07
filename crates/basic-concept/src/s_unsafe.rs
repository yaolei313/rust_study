//!
//! # study unsafe-rust
//!
//! unsafe代码块，没有内存安全保证的代码
//!
//! unsafe代码块的作用如下：
//! * 解引用裸指针 dereference a raw pointer
//! * 调用不安全的函数或方法
//! * 访问或修改**可变的**静态变量 mutable static variable
//! * 实现unsafe trait
//! * 访问union的字段
//!
//! unsafe关键字并不会关闭借用器检查或其它rust安全检查
//!
//!
//! data race:
//!   当一个线程访问一个可变对象时，另一个线程正在修改这个对象，就会发生数据竞争。

use core::slice;
use std::sync::atomic::Ordering;

///  
/// dereference a raw pointer
/// raw pointer允许
/// * 允许忽略借用规则，可以同时存在不可变和可变的指针，或者多个可变指针
/// * 不保证指向有效的内存
/// * 允许为空
/// * 没有自动清理功能
///
pub fn study_raw_point() {
    let mut num = 5;

    // 创建raw pointer不需要unsafe代码块，解引用需要unsafe代码块
    let p1 = &num as *const i32;
    let p2 = &mut num as *mut i32;

    unsafe {
        println!("value: {}, value: {}", *p1, *p2);
        // *p1 = 10;
        *p2 = 10;
        println!("value: {}, value: {}", *p1, *p2);
    }

    let address = 0x123456usize;
    let r = address as *const i32;

    // Ordering::SeqCst
    study_test1();
}

fn study_test1() {
    let mut v1 = vec![1, 2, 3];
    let p0 = &v1 as *const _ as *const u64;
    println!("address: 0x{:p} value: 0x{:x}", p0, unsafe { *p0 });
    // address: 0x0x7ffeebdf39b0 value: 0x7f9695c05c30

    for i in 0..100 {
        v1.push(i);
    }

    println!("address: 0x{:p} value: 0x{:x}", p0, unsafe { *p0 });
    // address: 0x0x7ffeebdf39b0 value: 0x7f9695c05cb0
    // Vec在stack中的内存结构是ptr,cap,len。ptr指向heap中的数据
    // 触发扩容，可能存在类似java ArrayList的机制。
}

pub fn study_call_unsafe() {
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // rust的borrow checker不能理解我们借用的是没有重叠的slice，只知道借用了slice 2次。借用没有重叠的slice是合理的，
    // 此时可以用unsafe block实现 safe abstraction。
    // cannot borrow `*values` as mutable more than once at a time
    // (&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 调用C代码
extern "C" {
    fn abs(p: i32) -> i32;
}

// C调用rust代码
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("call from clang");
}

///
/// 2个线程同时访问global variable（rust中也叫做static variable）会导致数据竞争
pub fn modify_gloal_variables() {
    println!("welcome! {}", HELLO_MSG);
    unsafe { COUNT += 1 };

    unsafe {
        println!("count:{}", COUNT);
    }
}

static HELLO_MSG: &str = "hello world";

static mut COUNT: u32 = 0;
