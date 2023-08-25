use std::net::TcpListener;

fn main() {
    // The function is called bind because, in networking, connecting to a port to listen to is known as “binding to a port.”
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("連線建立！");
    
    // 當 stream 離開作用域並在迴圈結尾被釋放時，連線會在 drop 的實作中被關閉。
    }
}
