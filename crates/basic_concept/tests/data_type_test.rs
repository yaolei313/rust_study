// rust只支持对library crate进行集成测试。
// 每一个 tests 目录中的测试文件都是完全独立的 crate
use basic_concept;

#[test]
fn test1() {
    println!("tests");
    assert_eq!(4, basic_concept::s_data_type::study_enum_convert());
}
