use std::ptr::NonNull;

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

#[cfg(test)]
mod test {
    use super::InlineBuf;

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
}
