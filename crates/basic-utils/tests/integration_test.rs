// rust只支持对library crate进行集成测试。
// 每一个 tests 目录中的测试文件都是完全独立的 crate
use basic_utils;

#[test]
fn it_adds_two() {
    println!("tests dir");
    assert_eq!(4, basic_utils::add(2, 2));
}
