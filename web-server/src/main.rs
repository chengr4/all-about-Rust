use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, sync::Arc,
};
fn main() {
    // The function is called bind because, in networking, connecting to a port to listen to is known as “binding to a port.”
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    
    // 當 stream 離開作用域並在迴圈結尾被釋放時，連線會在 drop 的實作中被關閉。
    }
}

fn handle_connection(mut stream: TcpStream) { 
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

    println!("請求：{:#?}", http_request);

}