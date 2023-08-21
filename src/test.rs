pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 测试模块，测试用例尽量写到这里边
#[cfg(test)]
mod add_function_tests {
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
