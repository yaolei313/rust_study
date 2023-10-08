// 每一个 tests 目录中的测试文件都是完全独立的 crate
use rust_study::s_unit_tes;

#[test]
fn it_adds_two() {
    println!("tests dir");
    assert_eq!(4, s_unit_test::add(2, 2));
}
