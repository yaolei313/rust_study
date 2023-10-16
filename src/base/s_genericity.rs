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

// 可以仅为特定泛型struct实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 为所有泛型struct实现方法
impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
}
