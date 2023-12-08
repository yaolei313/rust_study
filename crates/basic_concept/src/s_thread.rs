///
/// 竞态条件 race condition 事件的时序影响一段代码的正确性时，比如多个线程转账，账号A余额100，线程T1转出50，线程T2转出70。
/// 数据竞争 data race 一个线程读取一个可变数据时，另一个线程正在修改数据，若不同步，可能产生读写误差。
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub fn study_thread() {
    let t1 = thread::spawn(|| {
        for i in 1..10 {
            println!("number from the spawned thre'ad. {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    t1.join().unwrap();

    let v = vec![1, 2, 3];
    // move可以强制闭包获取其所使用环境的所有权
    let t2 = thread::spawn(move || {
        println!("here is a vec {:?}", &v);
    });
    t2.join().unwrap();
    // println!("{:?}", v); brorrowed moved value

    study_channel();

    study_mutex();
}

fn study_channel() {
    // mpsc multiple producer single consumer
    // 使用channel进行通信
    // tx: transmitter发送者缩写， rx：receiver缩写
    let (tx, rx) = mpsc::channel::<String>();
    // 通过clone创建多个producer
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("lisan"),
        ];
        for val in vals {
            // val move
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("lisan"),
        ];
        for val in vals {
            // val move
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let msg = rx.recv().unwrap();
    // 将rx当做一个Iterator对待
    for msg in rx {
        println!("receive message: {}", msg);
    }
}

fn study_mutex() {
    let v = vec![1, 2, 3];
    let m = Mutex::new(v);
    {
        let mut v1 = m.lock().unwrap();
        (*v1).push(4);
    }
    println!("m: {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut t = counter.lock().unwrap();
            *t += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("counter: {}", *counter.lock().unwrap());
}
