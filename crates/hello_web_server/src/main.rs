use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
};

use async_std::task;

const RESOURCES_PATH: &str = "resources/";

#[async_std::main]
async fn main() {
    let dir = std::env::current_dir().expect("get cwd fail");
    println!("dir: {}", dir.as_path().display());

    // 监听本地端口8080 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("127.0.0.1:8080").expect("bind :8080 failed");

    // 阻塞等待请求的进入
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                task::spawn(handle_connection(stream));
            }
            Err(e) => println!("connect fail {}", e),
        }
    }

    //     let pool = ThreadPool::new(4);
    //     let listener = TcpListener::bind("127.0.0.1:8080").expect("bind :8080 failed");
    //     for stream in listener.incoming() {
    //         let stream = stream.unwrap();
    //         println!("connection established");
    //         pool::execute(|| {
    //             handle_connection(stream);
    //         });
    //     }
    //     println!("Hello, world!");
}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();

    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("request {:?}", http_request);

    let (status_line, file_name) = if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else if http_request[0] == "GET /sleep HTTP/1.1" {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    write_response(stream, status_line, file_name);
}

fn write_response(mut stream: TcpStream, status_line: &str, file_name: &str) {
    let contents = fs::read_to_string(RESOURCES_PATH.to_string() + file_name).expect("read file fail");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("write response fail");
    stream.flush().expect("flush success");
}
