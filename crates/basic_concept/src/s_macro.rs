use hello_macro_derive::HelloMacro;

///
/// 声明宏declarative macro，代码替换
/// #[macro_export] 表明只要导入了包含这个macro的crate，该macro就是可用的。
///
/// 过程宏procedural macro，代码处理
///  - 自定义宏 #[derive] 在结构体或枚举上通过derive数学添加的代码
///  - 类属性宏
///  - 类函数宏
///
///
///
use crate::myvec;
use hello_macro::HelloMacro;

pub fn study_macro() {
    let v: Vec<u32> = vec![1, 2, 3];
    let v1 = myvec![2, 3, 4];
    println!("vec is {:?} {:?}", v, v1);

    Pancakes::hello_macro();
}

///
/// macro_rules！ 开始定义宏，宏名称不带！，使用的时候才带！
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

/// 自定义derive宏
#[derive(HelloMacro)]
pub struct Pancakes;
