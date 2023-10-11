// 生命周期声明属于函数签名的一部分
// 声明生命周期需要在<'a>内，比如&‘a i32,&'a mut i32，多个生命周期<'a, 'b>
// 'a所指代的生命周期为x,y生命周期重叠的那部分，也就是生命周期范围较小的那个
//
// 函数的输入参数的生命周期称为input lifetimes,返回参数的称为output lifetimes
// lifetime elision rule:
// 1.编译器为每个输入参数分配一个生命周期
// 2.若只有一个输入生命周期参数，则输出参数同生命周期
// 3.若若有多个输入生命周期参数，且包含&self，或&mut self，则输出参数生命周期同self的生命周期
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