use futures::FutureExt;
use tokio::{
    select,
    time::{sleep, Duration},
};

async fn task1() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("ss1")
}

async fn task2() -> String {
    sleep(Duration::from_secs(2)).await;
    String::from("ss2")
}

async fn study_select() {
    let mut t1 = task1().fuse();
    let mut t2 = task2().fuse();

    // <pattern> = <async expression> (, if <precondition>)? => <handler>,
    // tokio::select保证只有一个分支的结果处理被执行
    select! {
        result1 = t1 => {
            println!("future1 completed with result: {}", result1);
        }
        result2 = t2 => {
            println!("future2 completed with result: {}", result2);
        }
    }
}

pub fn study_tokio() {
    // study_select()
}
