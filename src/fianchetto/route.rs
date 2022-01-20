use route_recognizer::Params;

pub type RouteAction<T> = Box<
    dyn Fn(
            super::request::Request,
            &Params,
            Option<&T>,
        ) -> Result<String, Box<dyn std::error::Error>>
        + Send,
>;

pub struct Route<T> {
    pub path: String,
    pub method: String,
    pub action: RouteAction<T>,
}
