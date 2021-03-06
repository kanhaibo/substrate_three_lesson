// io操作引入预导入内容
use std::io::prelude::*;
// 网络监听绑定
use std::net::TcpListener;
// 网络流
use std::net::TcpStream;
// 文件操作
use std::fs;

fn main() {
    // 创建监听，并绑定端口号
    if let Ok(listener) = TcpListener::bind("127.0.0.1:7878") {
        // 始终遍历传进来的流序列的迭代器
        for stream in listener.incoming() {
            // 模式匹配当前流，成功则进行响应
            match stream {
                // 成功的流执行，读取动作
                Ok(stream) => {
                    hand_connection(stream);
                }
                // 返回错误的原因
                Err(e) => println!("the error {}", e.to_string()),
            }
        }
    }
    // 绑定失败，并打印错误信息
    else {
        println!("error to bind port !")
    }
}

// 读取请求并响应
fn hand_connection(mut stream: TcpStream) {
    // 声明存放数据的buffer
    let mut buffer = [0; 512];
    // 判断是否读取完整
    let mut done = true;
    // loop {
    //     let bytes_read = stream.read(&mut buffer).unwrap();
    //     if bytes_read < 512 {
    //         break;
    //     }
    // }

    // 读取内容到buffer
    while done {
        let bytes_read = stream.read(&mut buffer).unwrap();
        println!("{}", bytes_read);
        // 返回内容打印
        println!("Request:{}", String::from_utf8_lossy(&buffer[..]));
        if bytes_read < 512 {
            done = false;
        }
    }
    // 增加返回状态信息
    let contens = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK \r \n \r \n {}", contens);
    //写入返回流
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // 转化请求内容到字符串，并打印
}
