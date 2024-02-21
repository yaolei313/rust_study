use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{sync::atomic::AtomicU32, thread};

///
/// todo:
/// https://zhuanlan.zhihu.com/p/618127949
/// https://zhuanlan.zhihu.com/p/543559932
/// https://zhuanlan.zhihu.com/p/669908979
///
///
/// https://en.cppreference.com/w/cpp/atomic/memory_order
///
/// evaluation of each expression includes:
/// * value computations: 计算表达式返回的值
/// * initiation of side effects: 访问volatile glvalue修饰的对象，修改对象，io函数调用，或者调用任何执行这些操作的函数
///
/// 线性一致性Linearizability consistency
/// 所有线程看到的操作顺序，和全局时钟中操作的顺序一样。立即可见
///
/// 顺序一致性Sequential consistency
/// 一个线程的修改其它线程不一定立即可见；不同线程的写入操作，所有线程看到的顺序是一样的。
/// A sequenced-before B：同一个线程内，语言定义的执行顺序，A定义在前，即evaluation of A将在evaluation of B之前完成
///
///
///
/// Ordering枚举
/// relaxed 仅保证当前操作的原子性
/// acquire: 读操作load operation。当前线程的任何读或写操作都不能重排序到当前load操作前。其它线程的release the same atomic variable的在此操作前的所有写操作，当前线程可见。（保证后面操作的的顺序）
/// release: 写操作store operation。当前线程的任何读或写操作都不能重排序到当前load操作后。当前线程的所有写操作，在acquire the same atomic variable的其它线程中都可见（保证前面操作的顺序）
/// acq rel: 读写操作read-modify operation。当前线程的任何读写操作都不能重排序到load操作前，也不能重排序的store后。其它线程的release相同变量的在此操作前的所有写操作，当前线程可见。当前线程的写操作，在acquire相同变量的其它线程中可见(合并前2个)
/// seq cst: 多了全局顺序single total order，所有线程看到的修改顺序都是一致的。
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
