///
/// #ownership规则
/// each value in rust has a owner
/// there can only be one owner at a time
/// when the owner goes out of scope, the value will be dropped
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
/// 生命周期注解只是标明了多个引用间的生命周期的关系，并不改变其生命周期的长短。
///
/// 生命周期声明属于函数签名的一部分
/// 声明生命周期需要在<'a>内，比如&‘a i32,&'a mut i32，多个生命周期<'a, 'b>
///
/// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
/// 入参上的'a代表该参数的lifetime >= 'a，可以认为'a所指代的生命周期为x,y生命周期重叠的那部分(等于生命周期范围较小的那个范围)
/// 则出参的lifetime也是这个范围。生命周期声明不改变任何lifetime，
///
/// 参数的生命周期：输入生命周期input lifetimes
/// 返回的生命周期：输出生命周期output lifetimes
///
///
/// 编译器使用如下规则确认哪些场景可以不显示的标准生命周期，否则就需要显示指定生命周期。
/// 生命周期省略原则lifetime elision rules:
/// 1.入参中的每个引用分配一个生命周期 (应用于input lifetimes)
/// 2.若只有一个输入生命周期参数(只有一个引用类型)，则输出引用参数同生命周期 (应用于output lifetimes)
/// 3.若存在多个输入生命周期参数，但其中一个是&self，或&mut self，则输出参数生命周期同self的生命周期 (应用于output lifetimes)
///
/// 若编译器无法计算出output lifetime，则拒绝编译通过。
///
/// 若的确需要不同的lifetime，比如第3条，则需要显示指定下。
///
///
/// 匿名生命周期'_
/// 静态生命周期'static，和整个程序活的一样久，硬编码到二进制文件中
///
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 它的实际含义是 longest 函数返回的output lifetime与input lifetimes中lifetime较小的一致。
    // 注意 longest 函数并不需要知道 x 和 y 具体会存在多久，而只需要知道有某个可以被 'a 替代的作用域将会满足这个签名。

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn study_lifetimes() {
    let inst1 = ImportantExcerpt {
        local: String::from("libai"),
        part: "hello world",
    };
    let t3;
    {
        let t2 = "dufu";
        t3 = inst1.announce_and_return(t2);
    }
    println!("t3 {}", t3);
}

/*
fn study_lifetimes2() {
    let t2 = "dufu";
    let t3;
    {
        let inst1 = ImportantExcerpt {
            local: String::from("libai"),
            part: "hello world",
        };
        t3 = inst1.announce_and_return(t2);
    }
    // block 't  > inst1 'a > t2 'b , 故t3 'b
    println!("t3 {}", t3);
} */

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    local: String,
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, str: &str) -> &str {
        println!("{}", str);
        self.part
    }

    // 'a: 'b
    // lifetime 'a must liveout than lifetime 'b
    fn announce_and_return<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", self.part);
        announcement
    }
}
