//!
//! # study rust
//!
//! this is a libary crate for study rust.
//!

// 使用re-export避免crate使用者需要感知内部复杂的层次结构
pub use converter::convert_to_i32;

///
/// add two number
///
/// # Examples
///
/// ```
/// let sum1 = basic_utils::add(1,2);
/// assert_eq!(3, sum1);
///
/// ```
/// 
/// Output是associated type，可以在泛型声明处制定
///
pub fn add<T: std::ops::Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

///
/// return first world of &str
///
pub fn first_world(s: &str) -> &str {
    let bs = s.as_bytes();
    for (i, &item) in bs.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

pub mod converter {
    ///
    /// convert &str to i32, if err exists return default
    ///
    /// ```
    /// let r1 = basic_utils::convert_to_i32(0, "a");
    /// assert_eq!(0, r1);
    ///
    /// let r2 = basic_utils::convert_to_i32(0, "123");
    /// assert_eq!(123, r2);
    ///
    /// ```
    ///
    pub fn convert_to_i32(default: i32, str: &str) -> i32 {
        let r = str.trim().parse::<i32>();
        if r.is_err() {
            default
        } else {
            r.unwrap()
        }
    }
}

pub mod data_struct {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug, PartialEq, Eq)]
    pub struct BinaryTreeNode {
        pub left: Option<Rc<RefCell<BinaryTreeNode>>>,
        pub right: Option<Rc<RefCell<BinaryTreeNode>>>,
        pub val: i32,
    }

    impl BinaryTreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            BinaryTreeNode {
                left: None,
                right: None,
                val,
            }
        }

        #[inline]
        pub fn from(
            left: Option<Rc<RefCell<BinaryTreeNode>>>,
            right: Option<Rc<RefCell<BinaryTreeNode>>>,
            val: i32,
        ) -> Self {
            BinaryTreeNode { left, right, val }
        }

        #[inline]
        pub fn set_left(&mut self, l: &Rc<RefCell<BinaryTreeNode>>) {
            self.left = Some(Rc::clone(l));
        }

        #[inline]
        pub fn set_left_none(&mut self) {
            self.left = None;
        }

        #[inline]
        pub fn set_right(&mut self, l: &Rc<RefCell<BinaryTreeNode>>) {
            self.right = Some(Rc::clone(l));
        }

        #[inline]
        pub fn set_right_none(&mut self) {
            self.right = None;
        }
    }
}

pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.elements.len() == 0 {
            return None;
        }
        self.elements.get(self.elements.len() - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
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
