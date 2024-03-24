use futures::executor::block_on;

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
