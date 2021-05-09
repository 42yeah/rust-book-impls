mod thread_pool;

use std::net::TcpListener;
use http_server::handle_connection;
use thread_pool::ThreadPool;

fn main() {
    let thread_pool = ThreadPool::new(4);
    if let Ok(server) = TcpListener::bind("127.0.0.1:7878") {
        for stream in server.incoming().take(2) {
            let stream = stream.unwrap();
            thread_pool.execute(move || {
                handle_connection(stream);
            });
        }
        println!("Shutting down.");
    } else {
        panic!("Server binding failed");
    }
}
