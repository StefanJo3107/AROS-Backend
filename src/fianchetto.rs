pub mod request;
pub mod response;
pub mod route;

use crate::concurrency::ThreadPool;
use request::Request;
use route::Route;
use route_recognizer::{Params, Router};
use std::collections::HashMap;
use std::str;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

type MutRouter = Arc<Mutex<Router<Vec<Route>>>>;

pub struct Fianchetto {
    listener: TcpListener,
    pool: ThreadPool,
    router: MutRouter,
    routes: HashMap<&'static str, Vec<Route>>,
}

impl Fianchetto {
    pub fn new(address: &str, num_of_threads: usize) -> Self {
        let listener = TcpListener::bind(address).unwrap();
        let pool = ThreadPool::new(num_of_threads);
        let router = Router::new();
        let router = Arc::new(Mutex::new(router));
        let routes = HashMap::new();

        Fianchetto {
            listener,
            pool,
            router,
            routes,
        }
    }

    pub fn listen(&mut self) {
        self.set_router();

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let router = Arc::clone(&self.router);
            self.pool.execute(move || {
                handle_connection(stream, router);
            });
        }

        println!("Shutting down server!");
    }

    fn set_router(&mut self) {
        let mut router = self.router.lock().unwrap();
        for (path, routes) in self.routes.drain() {
            router.add(path, routes);
        }
    }

    fn request<F>(&mut self, path: &'static str, callback: F, method: String)
    where
        F: Fn(Request, &Params) -> String + Send + 'static,
    {
        let action = Box::new(callback);

        let route = Route {
            path: String::from(path),
            method,
            action,
        };

        if self.routes.contains_key(path) == false {
            let vec = vec![route];
            self.routes.insert(path, vec);
        } else {
            self.routes.get_mut(path).unwrap().push(route);
        }
    }

    pub fn get<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("GET"));
    }

    pub fn post<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("POST"));
    }

    pub fn put<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("PUT"));
    }

    pub fn delete<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("DELETE"));
    }
}

fn handle_connection(mut stream: TcpStream, router: MutRouter) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let buffer_str = str::from_utf8(&buffer).unwrap();

    let request = Request::new(buffer_str);

    let router = router.lock().unwrap();

    let route_match = router.recognize(request.path).unwrap();
    let routes: &Vec<Route> = route_match.handler();
    let mut response = String::from("");
    for route in routes {
        if route.method.eq(request.method) {
            response = (route.action)(request, route_match.params());
            break;
        }
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
