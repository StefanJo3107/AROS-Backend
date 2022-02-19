use route_recognizer::Params;

pub type RouteAction = Box<
    dyn Fn(super::request::Request, &Params) -> Result<String, Box<dyn std::error::Error>>
        + Send
        + Sync,
>;

pub struct Route {
    pub path: String,
    pub method: String,
    pub action: RouteAction,
}
