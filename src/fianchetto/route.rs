use route_recognizer::Params;

pub type RouteAction<T> =
    Box<dyn Fn(super::request::Request, &Params, Option<&T>) -> String + Send>;

pub struct Route<T> {
    pub path: String,
    pub method: String,
    pub action: RouteAction<T>,
}
