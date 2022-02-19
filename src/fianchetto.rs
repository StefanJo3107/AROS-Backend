pub mod request;
pub mod response;
pub mod route;

use crate::concurrency::ThreadPool;
use request::Request;
use route::Route;
use route_recognizer::{Params, Router};
use serde_json::json;
use std::collections::HashMap;
use std::str;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use self::response::Response;

type VecRouter = Router<Vec<Route>>;

pub struct Fianchetto {
    listener: TcpListener,
    pool: ThreadPool,
    routes: HashMap<&'static str, Vec<Route>>,
}

impl Fianchetto {
    pub fn new(address: &str, num_of_threads: usize) -> Self {
        let listener = TcpListener::bind(address).unwrap();
        let pool = ThreadPool::new(num_of_threads);
        let routes = HashMap::new();

        Fianchetto {
            listener,
            pool,
            routes,
        }
    }

    pub fn listen(&mut self) {
        let mut router = Router::new();
        self.set_router(&mut router);
        let arc_router = Arc::new(router);

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let router = Arc::clone(&arc_router);

            self.pool.execute(move || {
                if let Err(result) = handle_connection(stream, router) {
                    println!("Error: {}", result);
                }
            });
        }

        println!("Shutting down server!");
    }

    fn set_router(&mut self, router: &mut VecRouter) {
        for (path, routes) in self.routes.drain() {
            router.add(path, routes);
        }
    }

    fn request<F>(&mut self, path: &'static str, callback: F, method: String)
    where
        F: Fn(Request, &Params) -> Result<String, Box<dyn std::error::Error>>
            + Send
            + Sync
            + 'static,
    {
        let action = Box::new(callback);

        let route = Route {
            path: String::from(path),
            method,
            action,
        };

        if !self.routes.contains_key(path) {
            let vec = vec![route];
            self.routes.insert(path, vec);
        } else {
            self.routes.get_mut(path).unwrap().push(route);
        }
    }

    pub fn get<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> Result<String, Box<dyn std::error::Error>>
            + Send
            + Sync
            + 'static,
    {
        self.request(path, callback, String::from("GET"))
    }

    pub fn post<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> Result<String, Box<dyn std::error::Error>>
            + Send
            + Sync
            + 'static,
    {
        self.request(path, callback, String::from("POST"))
    }

    pub fn put<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> Result<String, Box<dyn std::error::Error>>
            + Send
            + Sync
            + 'static,
    {
        self.request(path, callback, String::from("PUT"))
    }

    pub fn delete<F>(&mut self, path: &'static str, callback: F)
    where
        F: Fn(Request, &Params) -> Result<String, Box<dyn std::error::Error>>
            + Send
            + Sync
            + 'static,
    {
        self.request(path, callback, String::from("DELETE"))
    }
}

fn handle_connection(
    mut stream: TcpStream,
    router: Arc<VecRouter>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 8192];
    stream.read(&mut buffer).unwrap();

    let buffer_str = str::from_utf8(&buffer)?;

    let request = Request::new(buffer_str)?;

    let route_match = router.recognize(request.path)?;
    let routes: &Vec<Route> = route_match.handler();
    let mut response = Response::bad_request();
    for route in routes {
        //preflight
        if request.method.eq("OPTIONS") {
            response = Response::no_content();
            break;
        } else if route.method.eq(request.method) {
            let response_res = (route.action)(request, route_match.params());
            match response_res {
                Ok(r) => response = r,
                Err(err) => {
                    let err = err.to_string();
                    let err_json = json!({ "err": err });
                    response = Response::bad_request_body(serde_json::to_string(&err_json)?);
                }
            }
            break;
        }
    }
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
