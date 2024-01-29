use super::s_struct_trait;
use std::thread;

// closure: anonymous functions that capture their enviroment.
pub fn study_closure() {
    // closure增加出入参类型标注
    let expensive_closure = |a: i32, b: i32| -> i32 { a + b };
    println!("2+3={}", expensive_closure(2, 3));

    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    let add_one_v3 = |x| x + 1;

    // 调用closure是能编译的必要条件，不然编译器没法推断类型。
    println!("add one 2 {}", add_one_v2(2));
    println!("add one 3 {}", add_one_v3(1));

    let mut list = vec![1, 2, 3];

    println!("Before defining closure1: {:?}", &list);
    // Closures can capture values from their environment in three ways,
    // borrowing immutably, borrowing mutably, and taking ownership.

    // the closure captures an immutable reference
    let only_borrows = || println!("From closure: {:?}", list);
    only_borrows();
    println!("After calling closure1: {:?}", list);

    println!("Before defining closure2: {:?}", list);
    // when `borrows_mutably` is defined, it captures a mutable reference to `list`
    let mut borrows_mutably = || list.push(7);
    // 这里不能immutable borrow 或new mutable borrow，因为已存在mutable borrow已存在，借出的可变引用的lifetime还没有结束。
    // 调用完成后，we don't use closure agin after the closure is called, so the mutable borrow ends
    borrows_mutably();
    println!("After calling closure2: {:?}", list);

    move_owner();
    study_fn_trait();
}

fn move_owner() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // 若希望所有权转移到closure，则需要使用move关键字
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

/// 一旦closure捕获了reference或者value(ownership)，closure body能做如下操作，move a captured value out of the closure, mutate captured value, 什么也不做
/// trait指定了struct或function能使用哪种closure。closure会依赖body中如何处理捕获的值，自动实现Fn trait。
///
/// 1.[FnOnce] 只能被调用一次的闭包，所有的闭包都至少实现了这个trait。若a closure that moves captured value out of its body will only implement FnOnce trait.
/// 2.[FnMut]:[FnOnce]  不会将所有权move出闭包体的闭包，但可能会修改对应的值。可以被多次调用。
/// 3.[Fn]:[FnMut]     不move out，不修改捕获的引用或变了。可以被多次调用。
/// 以上3种trait只是区分如何使用捕获的reference或value(move),不影响捕获逻辑
fn study_fn_trait() {
    // 1.若不需要捕获任何变量，可以直接使用函数名而不是闭包，比如
    let input = Some(vec![1, 2, 3]);
    unwrap_or_else(input, Vec::new);

    let mut list = [
        s_struct_trait::Rectangle {
            width: 10,
            height: 1,
        },
        s_struct_trait::Rectangle {
            width: 3,
            height: 5,
        },
    ];
    let mut count = 0;
    // pub fn sort_by_key<K, F>(&mut self, mut f: F)
    // where
    //     F: FnMut(&T) -> K,
    //     K: Ord,
    // ...
    // F定义的闭包类型，FnMut(&T) -> K, 当前类型的一个引用作为参数，返回一个可以排序的K类型。
    // 定义为FnMut是因为在排序过程中会多次调用这个闭包。
    // 且闭包 |r| r.width doesn’t capture, mutate, or move out anything from its environment, so it meets the trait bound requirements.
    // mut f: F,若闭包需要修改捕获的值，需要增加mut标识
    list.sort_by_key(|r| {
        count += 1;
        r.width
    });
    println!("{:#?}", list);

    let mut s = String::from("hello world");
    // 若需要在闭包内捕获可变引用，必须增加mut标识
    let mut modify_str = |str| s.push_str(str);
    modify_str(", libai");
    println!("FnMut {:?}", s);

    let modify2 = |str| s.push_str(str);
    let suffix = ", zhangsan.";
    exec(suffix, modify2);
    println!("FnMut2 {:?}", s);
}

// F定义了调用此函数传递的闭包类型，FnOnce() -> T 代表只能被调用一次，并返回一个T(move out)
fn unwrap_or_else<T, F>(input: Option<T>, f: F) -> T
where
    F: FnOnce() -> T,
{
    match input {
        Some(x) => x,
        None => f(),
    }
}

fn test2<T, F>(f: F, p: u32) -> T
where
    F: FnOnce(u32) -> T,
{
    f(p)
}

// 必须声明'a，不然会提示mismatched type
fn exec<'a, F>(input: &'a str, mut f: F)
where
    F: FnMut(&'a str) -> (),
{
    f(input)
}

fn exec2<F>(input: &str, mut f: F)
where
    F: FnMut(&str) -> (),
{
    f(input)
}
