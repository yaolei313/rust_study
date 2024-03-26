use std::fmt;

pub mod future_executor;
pub mod s_async;
pub mod s_others;
pub mod s_sync;
pub mod timer_future;

///
/// rust不检查trait的方法是否和自身方法冲突。故调用时需要使用fully qualified syntax调用
/// <Type as Trait>::function(receiver_if_method, next_arg, ...);
///
/// trait可以定义父子关系，这样可以要求impl trait A的类也实现B
/// trait B {}
/// trait A:B {}
///
/// 使用newtype pattern可以绕过coherence限制，比如为Vec<String>实现Display trait
///
/// ```text
/// pub struct Wrapper(Vec<String>);
///
/// impl fmt::Display for Wrapper {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "[{}]", self.0.join(", "));
///     }
/// }
/// ```
///
/// 可以通过type定义类型别名, 别名类型是synonym同义词。比如
/// type Kilometers = i32;
/// 任何i32可以出现的场景用类型别名替换都没有问题。
///
///
/// 返回类型!,称为never type。在函数从不返回的时候充当返回值，可以转换为任何类型。
///
/// Sized Trait代表编译时就可以确定大小。
/// 泛型参数默认只能用于编译时已知大小的类型，即fn generic<T>(t: T) {}实际代表fn generic<T: Sized>(t: T)
/// 若需要放宽这个限制，可以定义为fn generic<T: ?Sized>(t: &T) {} 代表T可能是也可能不是Sized。且参数t的类型不是Sized，那必须将其置于指针之后，此次选择是引用&T
///
/// 点操作符
/// 以<Type as Trait>::function(receiver_if_method, next_arg, ...)为例
/// 假设方法foo，有个接收器self，&self，&mut self，value的类型为T，那么value.foo()的逻辑如下：
///
/// * 尝试匹配T::foo(value)
/// * 尝试<&T>::foo(value) 和 <&mut T>::foo(value)
/// * 若T实现了Deref特征，T: Deref<Target = U>，则尝试使用U类型；若T未实现Deref，但是个定长类型，则尝试将定长类型转为不定长类型，比如[T;N]转为[T]
///
///
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn study_advanced() {
    println!("A baby dog is called a {}", Dog::baby_name());
    // full qualified syntax
    println!("full qualified syntax.{}", <Dog as Animal>::baby_name());

    // newtype pattern来解决trait impl的孤儿问题
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let a1 = do_twice(add_one, 5);
    assert_eq!(a1, 12);

    // 传递函数指针作为闭包
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("vec string: {:?}", list_of_strings);

    // u32类型，范围0..20。传递函数指针作为闭包
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("vec status: {:?}", list_of_statuses);
}

pub struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

/// 传递函数指针而不是闭包
/// fn是一个类型，是函数指针。fn类型实现了Fn，FnMut，FnOnce，所以所有出现函数闭包的地方，都可以传递函数指针作为参数
fn do_twice(f: fn(i32) -> i32, args: i32) -> i32 {
    f(args) + f(args)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
