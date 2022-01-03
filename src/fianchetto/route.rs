use route_recognizer::Params;

pub type RouteAction = Box<dyn Fn(Request, &Params) -> String + Send + 'static>;

pub struct Route {
    pub path: String,
    pub method: String,
    pub action: RouteAction,
}

pub struct Request;
