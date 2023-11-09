///
/// #ownership规则
/// * 每个value都有一个owner
/// * value在任意时刻只能有一个owner
/// * 当owner离开作用域，value将会被drop
///
/// 变量的作用域是函数结束，或者代码块结束
/// 引用的作用域是引用创建到最后一次使用。
///
/// 赋值，函数传入值，函数返回值(没有实现copy trait)都会导致所有权转移move。
///
/// move实际是浅复制(shallow copy)+旧value失效(value离开作用域会释放内存，旧value失效可以避免double free的问题)。
/// String 由栈中存储部分(ptr,len,capacity)和堆中存储部分组成。 move复制栈中部分。
///
/// copy trait类型的value会存储在栈上。
///
///
/// 什么时候需要指定生命周期
/// 生命周期就是引用的有效作用域，编译器检查不符合省略规则的时候，就需要手工指定。涉及场景struct和function,closure(anonymous function)
///
/// 生命周期注解只是标明了多个引用间的生命周期的关系，并不改变其生命周期的长短。
///
/// 生命周期声明属于函数签名的一部分
/// 声明生命周期需要在<'a>内，比如&‘a i32,&'a mut i32，多个生命周期<'a, 'b>
/// 'a所指代的生命周期为x,y生命周期重叠的那部分，也就是生命周期范围较小的那个
///
/// 编译器使用如下规则确认哪些场景可以不显示的标准生命周期。
/// 生命周期省略原则lifetime elision rule:
/// 1.每个引用参数分配一个生命周期
/// 2.若只有一个输入生命周期参数(只有一个引用类型)，则输出参数同生命周期
/// 3.若存在多个输入生命周期参数，且其中一个是&self，或&mut self，则输出参数生命周期同self的生命周期
///
/// 匿名生命周期'_
/// 静态生命周期'static，和整个程序活的一样久，硬编码到二进制文件中
///
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 它的实际含义是 longest 函数返回的引用的生命周期与函数参数所引用的值的生命周期的较小者一致。这些关系就是我们希望 Rust 分析代码时所使用的。

    // 记住通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝。
    // 注意 longest 函数并不需要知道 x 和 y 具体会存在多久，而只需要知道有某个可以被 'a 替代的作用域将会满足这个签名。

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    local: String,
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, str: &str) -> &str {
        println!("{}", str);
        self.part
    }
}
