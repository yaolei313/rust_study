use std::{
    cell::RefCell,
    fmt::Debug,
    ops::Deref,
    rc::{Rc, Weak},
    sync::Arc,
};

use rust_study::data_struct::BinaryTreeNode;

/// # Box<T>
/// 适用如下场景：
/// * 编译时未知大小的类型,Box作为智能指针，指向堆中数据
/// * 不被copy的场景下转移所有权
/// * 只关注是否实现了某个trait
/// Box<T> 指向堆上数据，当被丢弃时，会释放堆上的数据。单个所有者
///
/// Deref trait 重载了不可变引用的解引用操作符*，将智能指针当做常规引用处理。
/// DerefMut trait 重载了可变引用的解引用操作符*
/// Drop trait 值离开作用范围时执行的代码，比如是否网络连接，关闭文件等
///
/// Deref强制转换(Deref coercions)实现了将Deref trait的引用转换为另一种类型的引用。
/// 在调用函数或方法时，若实参和形参不同，且实参是实现了Deref trait类型的引用时，则编译时自动增加转换代码
/// T: Deref(target = U) , &T -> &U 或 &mut T -> &U
/// T: DerefMut(target = U) , &mut T -> &mut U
///
/// # Rc<T>
/// 引用计数智能指针，单线程环境使用。
/// Rc::new 会move ownership进入Rc，每次调用Rc::clone都会增加引用计数，离开作用域时减少引用计数
///
/// #Arc<T>
/// 多线程场景使用
///
/// # Cell<T>
/// 是不可变引用时，也可以修改内部的值，只能用于Copy trait类型
///
/// # RefCell<T>
/// 当RefCell是不可变引用时，也可以修改RefCell内部的值，只能用于非Copy trait类型
/// 运行期间执行可变借用检查，单线程环境使用
/// borrow返回Ref<T>,borrow_mut返回MutRef<T>.这2个类型均实现Deref，可以当做普通引用使用。
/// RefCell记录了不可变引用，可变引用的数量。从而允许在任意时刻只有一个可变引用或多个不可变引用。
///
///
///
/// Rc<T> 允许有多个所有者。Box<T> RefCell<T> 只允许单个所有者
///
pub fn study_smart_point() {
    let mut x: i32 = 5;
    modify(&mut x);

    println!("x: {}", x);

    let t = Box::new(6);

    // Box实现了Deref，可以当做常规引用对待。这里用MyBox模拟。
    let z = MyBox::new(5);

    // assert_eq!(5, z); 会提示can't compare integer with &integer
    // deref方法返回了希望通过*运算符的引用。没有Deref的话，编译器只会解引用&类型
    // *z 底层实际运行代码为 *(z.deref())
    assert_eq!(5, *z);

    // 提前drop值，不等作用域结束再调用Drop trait
    drop(z);

    let s = String::from("libai");
    let m1 = MyBox::new(s); // s moved,不能再使用
                            // 若实参和形参不同，且实参实现了Deref trait的引用类型，则编译时自动增加如下转换代码
                            // &MyBox<String> -> &String -> &str
    study_str_deref_coercion(&m1);

    study_rc();

    study_arc();

    study_ref_cell();
}

struct MyBox<T: Debug>(T);

impl<T: Debug> MyBox<T> {
    pub fn new(t: T) -> Self {
        MyBox(t)
    }
}

impl<T: Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> Drop for MyBox<T> {
    // 离开作用域时会被调用，但不允许手工提前调用
    fn drop(&mut self) {
        println!("drop mybox with data `{:?}`", self.0);
    }
}

fn study_str_deref_coercion(s: &str) {
    println!("hello {s}");
}

fn modify(number: &mut i32) {
    *number += 1;
}

pub fn study_rc() {
    let n6 = Rc::new(RefCell::new(BinaryTreeNode::new(6)));

    let n5 = Rc::new(RefCell::new(BinaryTreeNode::new(5)));
    let n4 = Rc::new(RefCell::new(BinaryTreeNode::new(4)));

    let n2 = Rc::new(RefCell::new(BinaryTreeNode::from(
        Some(Rc::clone(&n4)),
        Some(Rc::clone(&n5)),
        2,
    )));

    let n3 = Rc::new(RefCell::new(BinaryTreeNode::from(
        Some(Rc::clone(&n6)),
        None,
        3,
    )));

    let root = Rc::new(RefCell::new(BinaryTreeNode::from(
        Some(Rc::clone(&n2)),
        Some(Rc::clone(&n3)),
        1,
    )));
    let t = Rc::clone(&root);
    bfs_tree(Some(t));

    println!("{}", root.borrow().val);
}

fn bfs_tree(root: Option<Rc<RefCell<BinaryTreeNode>>>) {
    if root.is_none() {
        return;
    }
    let mut q = Vec::new();
    q.push(root.unwrap());
    while !q.is_empty() {
        let size = q.len();
        for _i in 0..size {
            let t = q.remove(0);
            let t1 = t.borrow();
            print!("\t {}", t1.val);

            if let Some(l) = t1.left.as_ref() {
                q.push(Rc::clone(l));
            }
            if let Some(r) = t1.right.as_ref() {
                q.push(Rc::clone(r));
            }
        }
        println!();
    }
    println!("bfs end");
}

fn study_arc() {
    let a = Arc::new(5);
    let b = Arc::clone(&a);
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

fn study_ref_cell() {
    let a: Rc<_> = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail()); // Some(RefCell{Rc{nil}}))

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // Some(RefCell{Rc{Cons(5, RefCell{Rc{nil}})}})

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // a.tail = b, b.tail = a
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    println!("=========================");
    let leaf = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf),   // 0
    );

    {
        let branch = Rc::new(TreeNode {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch),   // 1
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf),   // 0
        );
        // Rc<T>实现了Drop trait，branch离开作用域时
        // branch被释放，leaf的strong count也减1
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!("=========================");

    let a = RefCell::new(5);
    let mut val = a.borrow_mut();
    *val += 10;
    // 只有引用的作用域是最后一次使用位置。非引用的生命周期是变量作用域。
    drop(val);
    println!("after modify {}", a.borrow());
}
