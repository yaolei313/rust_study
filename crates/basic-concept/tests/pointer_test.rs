#[cfg(test)]
mod test {
    use std::{cell::{Cell, OnceCell}, thread};

    use basic_concept::s_pointer;

    #[test]
    fn test1() {
        s_pointer::study_pointer();
    }

    #[test]
    fn test2() {
        let c = Cell::new("asdf");
        let one = c.get();
        c.set("qwer");
        let two = c.get();
        println!("{}, {}", one, two);

        let mut c2 = Cell::new(String::from("value"));
        let m1 = c2.get_mut();
        //let m2 = c2.get_mut();
        m1.push_str("abc");
        // m2.push_str("efg");
        println!("{:?}", c2.take());
    }

    #[test]
    fn test3() {
        let mut c = OnceCell::<String>::new();
        if let Ok(_) = c.set(String::from("value")) {
            println!("set success");
        }
        c.get_mut().map(|item| item.push_str(" hello world"));
        let v = c.get().expect("get value fail");
        println!("{}", v);
    }

    #[test]
    fn test4() {
        let c = OnceCell::<String>::new();
        if let Ok(_) = c.set(String::from("value")) {
            println!("set success");
        }
        let v = c.get().expect("get value fail");
        println!("{}", v);
    }

}
