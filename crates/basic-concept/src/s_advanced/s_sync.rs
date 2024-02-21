use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{sync::atomic::AtomicU32, thread};

///
/// todo:
/// https://zhuanlan.zhihu.com/p/618127949
/// https://zhuanlan.zhihu.com/p/669908979
/// 
///
///
/// https://en.cppreference.com/w/cpp/atomic/memory_order
///
/// evaluation of each expression includes:
/// * value computations: 计算表达式返回的值
/// * initiation of side effects: 访问volatile glvalue修饰的对象，修改对象，io函数调用，或者调用任何执行这些操作的函数
///
/// 线性一致性Linearizability consistency
/// 在顺序一致性的基础上,增加如下限制
/// 条件3: 不同进程的事件，如果在时间上不重叠，即不是并发事件，那么要求这个先后顺序在重排之后保持一致。 
///
/// 顺序一致性Sequential consistency
/// 条件1: Each processor issues memory requests in the order specified by its program.
///       每个进程执行内存访问的顺序和程序顺序一致 
/// 条件2: Memory requests from all processors issued to an individual memory module are serviced from a single FIFO queue. 
///       Issuing a memory request consists of entering the request on this queue. 
///       所有进程执行内存的访问要表现得像 FIFO 一样先入先出，每次读到的都是最近写入的值
/// 
/// A sequenced-before B：同一个线程内，evaluation of A将在evaluation of B之前完成
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
