use futures::{channel::mpsc, executor::block_on, Sink, SinkExt, StreamExt};

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
