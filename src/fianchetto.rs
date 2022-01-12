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

use self::response::Response;

type MutRouter<T> = Arc<Mutex<Router<Vec<Route<T>>>>>;

pub struct Fianchetto<T> {
    listener: TcpListener,
    pool: ThreadPool,
    router: MutRouter<T>,
    routes: HashMap<&'static str, Vec<Route<T>>>,
    db_conn: Arc<Mutex<Option<T>>>,
}

impl<T> Fianchetto<T>
where
    T: Send + 'static,
{
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
            db_conn: Arc::new(Mutex::new(None)),
        }
    }

    pub fn listen(&mut self) {
        self.set_router();

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let router = Arc::clone(&self.router);
            let conn = Arc::clone(&self.db_conn);

            self.pool.execute(move || {
                handle_connection(stream, router, conn);
            });
        }

        println!("Shutting down server!");
    }

    pub fn set_db_connection(&mut self, db_conn: T) {
        self.db_conn = Arc::new(Mutex::new(Some(db_conn)));
    }

    fn set_router(&mut self) {
        let mut router = self.router.lock().unwrap();
        for (path, routes) in self.routes.drain() {
            router.add(path, routes);
        }
    }

    fn request<F>(&mut self, path: &'static str, callback: F, method: String)
    where
        F: Fn(Request, &Params, Option<&T>) -> String + Send + 'static,
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
        F: Fn(Request, &Params, Option<&T>) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("GET"));
    }

    pub fn post<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params, Option<&T>) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("POST"));
    }

    pub fn put<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params, Option<&T>) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("PUT"));
    }

    pub fn delete<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params, Option<&T>) -> String + Send + 'static,
    {
        self.request(path, callback, String::from("DELETE"));
    }
}

fn handle_connection<T>(
    mut stream: TcpStream,
    router: MutRouter<T>,
    db_conn: Arc<Mutex<Option<T>>>,
) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let buffer_str = str::from_utf8(&buffer).unwrap();

    let request = Request::new(buffer_str).unwrap();

    let router = router.lock().unwrap();
    let db_conn = db_conn.lock().unwrap();
    let db_conn = db_conn.as_ref();

    let route_match = router.recognize(request.path).unwrap();
    let routes: &Vec<Route<T>> = route_match.handler();
    let mut response = Response::bad_request();
    for route in routes {
        if route.method.eq(request.method) {
            response = (route.action)(request, route_match.params(), db_conn);
            break;
        }
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
