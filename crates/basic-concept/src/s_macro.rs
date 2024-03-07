use hello_macro_derive::{route, sql, HelloMacro};

///
/// 声明宏declarative macro
/// 通过 #[macro_export] 表明
/// 只要导入了包含这个macro的crate，该macro就是可用的。
///
/// 过程宏procedural macro
///  - 自定义派生宏 #[derive(XxxName)] 在结构体或枚举上通过宏添加代码
///  - 类似属性宏attribute-like macro
///  - 类似函数宏function-like macro
///
/// 过程宏从输入中获得标记流，处理然后生成新的标记流，然后把标记流交给编译器继续处理。过程宏操作的是rust ast
///
use crate::myvec;
use hello_macro::HelloMacro;

pub fn study_macro() {
    let v: Vec<u32> = vec![1, 2, 3];
    let v1 = myvec![2, 3, 4];
    println!("vec is {:?} {:?}", v, v1);

    // derive macro
    Pancakes::hello_macro();

    // function-like macro
    // let sql = sql!(select * from user where id=1);
}

/// 声明宏：使用macro_rules!定义
///
/// 开始定义宏，宏名称不带！，使用的时候才带！
#[macro_export]
macro_rules! myvec {
    // 分支模式，若是匹配，则执行=>之后的内容
    // $表示是宏变量，()代表捕获和模式匹配的代码。$($x:expr)内$x:expr,其匹配任何rust的代码，并命名为$x
    // ,* 代表可以匹配1个或多个之前的模式
    // 每匹配一次都会在右侧$()*中的内容重复1次
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 过程宏：自定义派生宏，#[derive(Xxxx)]
#[derive(HelloMacro)]
pub struct Pancakes;

// 过程宏：attribute-like宏
// #[route(GET, "/")]
fn index() {
    todo!()
}
