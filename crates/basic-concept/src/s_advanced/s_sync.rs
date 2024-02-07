use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{sync::atomic::AtomicU32, thread};

///
/// Ordering枚举
///
///
///
pub fn study_sync() {
    let cur_count = Arc::new(AtomicU32::new(0));

    let mut threads = Vec::with_capacity(10);
    for _i in 0..10 {
        let cc = Arc::clone(&cur_count);
        let handle = thread::spawn(move || {
            cc.fetch_add(1, Ordering::Relaxed);
        });
        threads.push(handle);
    }

    for t in threads {
        t.join().expect("join fail");
    }
    println!("{}", cur_count.load(Ordering::Relaxed));
}
