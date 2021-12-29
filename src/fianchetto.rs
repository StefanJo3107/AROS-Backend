pub mod route;

use crate::concurrency::ThreadPool;
use route::{Request, Response, Route};
use route_recognizer::{Params, Router};

use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct Fianchetto {
    listener: TcpListener,
    pool: ThreadPool,
    router: Router<Route>,
}

impl Fianchetto {
    pub fn new(address: &str, num_of_threads: usize) -> Self {
        let listener = TcpListener::bind(address).unwrap();
        let pool = ThreadPool::new(num_of_threads);
        let router = Router::new();

        Fianchetto {
            listener,
            pool,
            router,
        }
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            self.pool.execute(move || {
                handle_connection(stream);
            });

            println!("Shutting down server!");
        }
    }

    pub fn get<F>(&mut self, path: &str, callback: F)
    where
        F: Fn(Request, Params) -> Response + 'static,
    {
        let action = Box::new(callback);
        self.router.add(
            path,
            Route {
                path: String::from(path),
                method: route::HTTPMethod::GET,
                action,
            },
        );
    }

    pub fn post() {}

    pub fn put() {}

    pub fn delete() {}
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "demo.json")
    } else if buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK", "demo.json")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "demo.json")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
