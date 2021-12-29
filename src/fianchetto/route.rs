use route_recognizer::Params;

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub type RouteAction = Box<dyn Fn(Request, Params) -> Response + 'static>;

pub struct Route {
    pub path: String,
    pub method: HTTPMethod,
    pub action: RouteAction,
}

pub struct Request;
pub struct Response {}

impl Response {
    pub fn ok() -> Self {
        Response {}
    }
}
