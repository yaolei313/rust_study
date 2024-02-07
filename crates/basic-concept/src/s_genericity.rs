use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Add;

///
/// 泛型是在编译时执行static dispatch，也就是单态化，为每个被泛型替代的具体类型生成非泛型实现。
/// dyn trait是运行时执行dynamic dispatch，在运行时确定调用的代码。
///   -- trait对象是个胖指针，包含2个指针，data pointer,vtable pointer。使用vtable指针指向的数据可以确定trait的具体类型。
///
/// 泛型有个语法叫默认类型参数default type parameters，比如
/// pub trait Add<Rhs=Self> {
///     type Output;
///     fn add(self, other:Rhs) -> Self::Output;
/// }
/// 即实现时，若不指定Rhs的类型，则默认为Self的类型。
///
#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 可以仅为特定泛型struct实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 为所有泛型struct实现方法
impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

pub fn study_genericity() {
    let p1 = Point::new(10, 20);
    println!("x: {}, y:{}", &p1.get_x(), p1.get_y());
    // p1无法调用distance_from_origin方法

    let p2 = Point {
        x: 3.0 as f32,
        y: 4.0 as f32,
    };
    println!("distance: {}", p2.distance_from_origin());

    let a1 = [12; 5];
    let a2 = [1, 2, 3, 4, 5, 6, 7];
    display_array2(a1);
    display_array2(a2);
}

pub fn display_array<T: Display + Debug>(t: &[T]) {
    println!("{:?}", t);
}

// const针对值的泛型
pub fn display_array2<T: Debug, const N: usize>(t: [T; N]) {
    println!("{:?}", t);
}
