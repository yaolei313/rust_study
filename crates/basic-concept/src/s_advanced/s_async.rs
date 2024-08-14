use futures::{channel::mpsc, executor::block_on, select, FutureExt, Sink, SinkExt, StreamExt};

///
/// async function被调用时不会被执行，而是capture arguments into a future
/// [std::future::IntoFuture]
///
/// await
///
/// future是惰性的，必须调用await(poll)才会执行
///
/// await
/// Suspend execution until the result of a [Future] is ready.
/// .awaiting a future will suspend the current function's execution until the executor has run the future to completion.
/// Read the async book for details on how async/await and executors work.
///
/// [futures::join!]      并发运行future，等待全部完成
/// [futures::try_join!]  并发运行future，但一个future报错立即返回
/// [futures::select!]    并发运行future，一个完成即可以被处理。待学习。
///                       模式 = async 表达式 => 结果处理，并发执行所有分支的async表达式，任一执行完成，和模式进行匹配，若匹配，则剩下的表达式会被释放
///
pub fn study_async() {
    let f1 = do_println();
    let f2 = do_compute(10);
    block_on(f1);
    let r2 = block_on(f2);
    println!("{}", r2);
}

async fn do_println() {
    do_step1().await;
    println!("hello async world");
}

async fn do_step1() {
    println!("step1")
}

async fn do_compute(p: i32) -> i32 {
    p + 1
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    // 必须把SinkExt引入作用域才可以使用
    println!("1");
    tx.send(1).await.unwrap();
    println!("2");
    tx.send(2).await.unwrap();
    println!("3");
    // tx.try_send(3);
    drop(tx);

    // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
    // 因此还需要使用`.await`来获取具体的值
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

async fn foo() -> Result<u8, String> {
    Ok(1)
}
async fn bar() -> Result<u8, String> {
    Ok(1)
}
pub fn func1() {
    let fut = async {
        foo().await?;
        bar().await?;
        // Ok(()) 此次使用?号时，编译器不能推断Result<T,E>中的类型
        Ok::<(), String>(())
    };
}

async fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n
    } else {
        Box::pin(fib(n - 1)).await + Box::pin(fib(n - 2)).await
    }
}

use std::sync::{Mutex, MutexGuard};

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;

    foo().await;
} // 锁在这里超出作用域
