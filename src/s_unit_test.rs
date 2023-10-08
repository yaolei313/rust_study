pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 测试模块，测试用例尽量写到这里边
// assert!
// assert_eq!
// assert_ne!
// #[cfg(test)]告诉rust编译器只有在cargo test执行的时候才编译和运行测试代码，而执行cargo build时不执行
#[cfg(test)]
mod add_function_tests {
    // 引用父模块的所有项
    use super::*;

    // 单元测试
    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 3), 7)
    }

    #[test]
    #[ignore = "not yet reviewed by the Q.A. team"]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}
