///
/// 模式irrefutable 不可反驳的，能匹配任何传递的可能值的模式，比如let x = 5; x可以匹配任何值，匹配不会失败。
/// 模式refutable 可反驳性，可能匹配失败的模式，称为可反驳的，比如if let Some(x)=a_value; 若a_value是None则Some(x)不能匹配。
///
/// 函数参数，let语句，for循环 只能接受irrefutable pattern。
/// if let,while let 可以接受retutable pattern和irrefutable pattern。
///
/// match guard是match pattern之后的一段if语句。
///
/// match一般都有穷尽性检查，match + match guard就没有了。
/// if let的多层组合没有穷尽性检查
///
/// match中的变量屏蔽不容易看出，所以尽量使用不同的变量名
///
pub fn study_pattern_match() {
    // let中的模式匹配，解构元组
    let (x, y, z) = (1, 2, 3);

    let (a, b, _) = (4, 5, 6);

    let (a, b, ..) = (1, 2, 3, 4, 5, 7);

    // match表达式必须是穷尽的exhaustive
    // match模式中的other和_,other[任意名称]会匹配任意值并绑定值，_匹配任意值但忽略绑定值
    let roll = 9;
    let mut t1 = String::from("e");
    let r2: &str = match roll {
        1 => "a",
        2 => "b",
        other => {
            let t2 = other.to_string();
            t1.push_str(&t2);
            t1.as_str()
        }
    };
    println!("r2 is {}", r2);

    // 可以用|匹配多个模式
    // 可以通过..=指定值的范围,只允许数字类型和char类型
    match roll {
        3 | 4 => println!("3 or 4"),
        5..=10 => println!(">= 5 && <= 10"),
        12 => {
            println!("12")
        }
        _ => (),
    }

    let a_number = Some(8);
    // if let表达式，单个模式匹配
    if let Some(v) = a_number {
        println!("value is {}", v);
    } else {
        println!("other")
    }
    assert_eq!(a_number.unwrap(), 8);

    // if let可以组合，并不要求条件相互关联。相比match，少了exhaustive检查（穷尽性检查，检查是否包含所有的可能性）
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let只要模式匹配，一直进行while循环
    let mut s = vec![1, 2, 3];
    while let Some(item) = s.pop() {
        println!("{}", item);
    }

    // 解构结构体，变量a,b匹配，变量名可以和字段名一致，从而可以简写
    let p = Point { x: 1, y: 5 };
    let Point { x: a, y: b } = p;
    println!("{} {}", a, b);

    let Point { x, y } = p;
    println!("{} {}", x, y);

    // 匹配守卫match guard，编译器不会检查穷尽性
    let x = Some(5);
    match x {
        Some(a) if a % 2 == 0 => {
            println!("x is even")
        }
        Some(a) => println!("x is odd"),
        None => println!("none"),
    }

    // match guard中使用|操作符
    let y = 4;
    let c = false;
    match y {
        2 | 4 | 6 if c => {
            println!("matched")
        }
        _ => println!("not matched"),
    }

    // @绑定
    let msg = Message::Hello { id: 10 };
    match msg {
        Message::Hello {
            id: id_other_name @ 3..=10,
        } => {
            // 创建一个变量存放满足@之后条件的值
            println!("range >=3 && <=10 {}", id_other_name)
        }
        Message::Hello { id: 11..=15 } => {
            // 没有将id保存进一个变量内
            println!("range >=11 && <= 15")
        }
        Message::Hello { id } => {
            println!("other value {}", id)
        }
    }

    // matches!宏
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // rust中的枚举不能像java一样直接使用==判断，得使用matches!宏，也就是match逻辑
    // v.iter().filter(|x| **x == MyEnum::Foo);
    let v2 = v.iter().filter(|x| matches!(x, MyEnum::Foo));

    let foo = 's';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    // 等同如下逻辑，可以看下matches宏定义
    let r: bool = match foo {
        'A'..='Z' | 'a'..='z' => true,
        _ => false,
    };
    assert!(r);

    // match guard
    let t1 = Some(3);
    assert!(matches!(t1, Some(i) if i>2));

    // 匹配值的范围，比使用 1|2|3 => ... 等方便
    let t = 3;
    match t {
        1..=10 => println!("less than or equal 10"),
        _ => println!("more than 10"),
    }

    match t {
        x if x > 0 && x < 10 => println!("range (0,10)"),
        _ => println!("others"),
    }

    match t {
        x if x == 1 || x == 2 => println!("1 or 2 {}", x),
        _ => println!("others"),
    }

    // 解构数组
    let a1 = [32, 36];
    let [ax, ay] = a1;

    let a2 = [1, 2, 3, 4, 5, 6];
    let [a2f, ..] = a2;

    let a3 = &[1, 2, 3, 4, 5, 6];
    let [a3f, ..] = a3;

    // @绑定
    let t2 = Some(33);
    match t2 {
        Some(var1 @ 1..=100) => println!("val {}", var1),
        Some(_) => println!("other"),
        None => println!("none"),
    };

    // @绑定等同如下逻辑
    match t2 {
        Some(var1) if var1 >= 1 && var1 <= 100 => println!("val {}", var1),
        Some(_) => println!("other"),
        None => println!("none"),
    };
}

// 模式匹配匹配元组
fn test_patter_param(&(x, y): &(i32, i32)) {
    println!("{}", x * y);
}

fn test_patter_param2((x, y): (i32, i32)) {
    println!("{}", x * y);
}

pub struct Point {
    x: i32,
    y: i32,
}

pub enum Message {
    Hello { id: i32 },
}

enum MyEnum {
    Foo,
    Bar,
}
