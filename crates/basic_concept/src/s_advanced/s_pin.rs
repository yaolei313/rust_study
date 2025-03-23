use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

///
/// Pin<Ptr<T>> Ptr所指向的T在内存中固定住，不能移动。
/// 也就是不能通过safe代码拿到&mut T, 从而无法修改，也就不会导致T的memory location发生变化
///
/// Pin可以被看作一个限制指针&mut T的结构
/// 在T: Unpin的情况下，通过DerefMut就可以直接得到&mut T，
/// 在T: !Unpin的情况下，Pin<T>只能通过Deref得到&T，就是说T被pin住了
///
/// Pin<Ptr<T:Unpin>>U的使用 TODO
///
/// [PhantomPinned]是maker type，任意type包含PhantomPinned属性，则该type默认negative impl [Unpin]
///
/// 若T: !Unpin 才可以被pin住
///
/// Pin实际是对P指针的限制，在T没有实现Unpin的情况下，避免P指针暴露&mut Self。
/// Pin的引入是Async/.Await异步编程的需要，核心就是Future::poll方法参数的需要。故除了Future::poll方法之外，不建议使用Pin，也没有必要使用Pin.
///
#[derive(Debug)]
pub struct InlineBuf {
    data: [u8; 32],
    ptr: NonNull<[u8]>,
}

impl InlineBuf {
    pub fn new() -> Self {
        Self {
            data: [0; 32],
            ptr: NonNull::from(&[]),
        }
    }

    pub fn set_content(&mut self, bs: &[u8]) {
        if bs.len() > self.data.len() {
            return;
        }
        self.data[0..bs.len()].copy_from_slice(bs);
        self.ptr = NonNull::from(&self.data);
    }

    pub fn get_content(&self) -> &str {
        let bs = unsafe { &*self.ptr.as_ptr() };
        std::str::from_utf8(bs).expect("fail")
    }
}

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _mark: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _mark: Default::default(),
        }
    }

    fn init1(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a1(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ref: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ref;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }
}

#[cfg(test)]
mod test {
    use super::{InlineBuf, Test};

    #[test]
    fn test1() {
        let mut t1 = InlineBuf::new();
        let addr1 = &t1 as *const InlineBuf as usize;
        println!("addr1: {}", addr1);
        {
            let mut t2 = InlineBuf::new();
            t2.set_content(b"hello");
            let addr2 = &t2 as *const InlineBuf as usize;
            println!("addr2: {}", addr2);

            // 栈上值复制
            t1 = t2;
            println!("t1: {:?},data1: {:?}", t1, t1.get_content());

            t2 = InlineBuf::new();
            let addr2 = &t2 as *const InlineBuf as usize;
            println!("addr2: {}", addr2);
            t2.set_content(b"world");
        }
        let addr1 = &t1 as *const InlineBuf as usize;
        println!("addr1: {}", addr1);
        println!("t1: {:?},data1: {:?}", t1, t1.get_content());
    }

    #[test]
    fn test3() {
        // let mut test1 = Test::new("test1");
        // test1.init();
        // let mut test2 = Test::new("test2");
        // test2.init();
        //
        // std::mem::swap(&mut test1, &mut test2);
        //
        // println!("a: {}, b: {}", test1.a(), test1.b());
        // println!("a: {}, b: {}", test2.a(), test2.b());
    }
}
