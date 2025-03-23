// rust只支持对library crate进行集成测试。
// 每一个 tests 目录中的测试文件都是完全独立的 crate
use basic_utils;

#[test]
fn it_adds_two() {
    println!("tests dir");
    assert_eq!(4, basic_utils::add(2, 2));
}

#[test]
fn test_drop() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    println!("Running --- 1");
    let _foo = Foo;
    println!("Running --- 2");
}

struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}
