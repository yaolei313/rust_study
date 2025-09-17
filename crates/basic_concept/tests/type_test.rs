// rust只支持对library crate进行集成测试。
// 每一个 tests 目录中的测试文件都是完全独立的 crate
#[cfg(test)]
mod test {
    use basic_concept::s_type;
    use std::hash::Hash;

    #[test]
    fn test0() {
        s_type::study_primitive_type();
    }

    #[test]
    fn test1() {
        s_type::study_enum_convert();
    }

    #[test]
    fn test2() {
        basic_concept::s_advanced::s_others::study_format();
    }

    #[test]
    fn test3() {
        let s = String::from("hello world");
        let a1 = &s;

        print_args(a1);

        let c = String::from("c");

        print_args(a1);

        let p = a1 as *const _ as *const u64;
        println!("address: {:p} {}", p, unsafe { *p });
        println!("{c}");
    }

    fn print_args(s: &str) {
        let p: *const u64 = s as *const _ as *const u64;
        println!("{:p}", p);
    }

    #[test]
    fn test4() {
        // let mut s = String::from("hello"); // CFE2608

        // let r1 = &s; // no problem CFE2620
        // let r2 = &s; // no problem CFE2628

        // println!("{:p}, {:p}", &r1, &r2);

        let mut p = Point { x: 10 }; // a1ec6e4
        let t1 = &mut p; // a1ec6f8
        println!("{:p}", &t1);
        Point::add(t1, 10); // a1ec560
        Point::add(t1, 11);
        Point::read(t1);
        Point::read(t1);
    }

    #[derive(Debug)]
    struct Point {
        x: i32,
    }

    impl Point {
        fn add(&mut self, other: i32) {
            let p = &self as *const _ as *const u64;
            println!("address1: {:p} {:x}", p, unsafe { *p });
            self.x += other;
        }

        fn read(&mut self) -> i32 {
            let p = &self as *const _ as *const u64;
            println!("address2: {:p} {:x}", p, unsafe { *p });
            self.x
        }
    }

    #[test]
    fn test5() {
        let mut p = String::from("hello");

        let t1 = &mut p;
        let t2;
        t2 = t1; // &T 是move，move后t1失效

        // println!("{:?}", t1);
        let c1 = 'a';
        println!("c1 size: {}", size_of_val(&c1));
    }

    /// [Copy]
    ///
    #[test]
    fn test6() {
        let s = String::from("hello");
        let s2 = String::from("world");

        let t1 = &s;
        let t2 = t1; // &T 是copy

        println!("{:?} {:?}", t1, t2);
    }

    #[test]
    fn test7() {
        let s = "hello";
        let s2 = String::from(s);

        println!("s2 {} s {}", s2, s);
    }

    #[test]
    fn test8() {
        s_type::study_slice();
    }

    #[test]
    fn test_study_vec() {
        let mut vec1 = vec![Point { x: 10 }, Point { x: 11 }, Point { x: 12 }];
        println!("{:?}", vec1);
        let item1 = vec1.pop();
        println!("{:?} {:?}", vec1, item1);
        let item2 = vec1.remove(1);
        println!("{:?} {:?}", vec1, item2);
    }

    #[test]
    fn test9() {

    }

    pub trait Name {
        fn print(&self);
    }

    pub struct Car {

    }

    impl Name for Car {
        fn print(&self) {
            println!("car");
        }
    }

    pub struct Hold<T: Name> {
        hold: Option<T>,
    }

    impl<T: Name> Hold<T> {
        pub fn view(&self) {
            if let Some(ref n) = self.hold {
                n.print()
            }
        }
    }
}
