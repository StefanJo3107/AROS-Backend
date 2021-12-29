mod concurrency;
mod fianchetto;

use fianchetto::route::Response;
use fianchetto::Fianchetto;

fn main() {
    let mut app = Fianchetto::new("127.0.0.1:1207", 4);

    app.get("/turnir", |_, _| {
        println!("Route was hit successfully!");
        Response::ok()
    });

    app.listen();
}
