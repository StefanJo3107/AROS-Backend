mod concurrency;
mod fianchetto;

use fianchetto::response::Response;
use fianchetto::Fianchetto;

fn main() {
    let mut app = Fianchetto::new("127.0.0.1:1207", 4);

    app.get("/turnir", |_, _| {
        println!("Get was sent successfully!");

        Response::ok(String::from(""))
    });

    app.post("/turnir", |_, _| {
        println!("Post was sent successfully!");

        Response::ok(String::from(""))
    });

    app.put("/turnir/:id", |_, params| {
        if let Some(id) = params.find("id") {
            println!("Put was sent successfully with id: {}", id);
        }

        Response::ok(String::from(""))
    });

    app.delete("/turnir/:id", |_, params| {
        if let Some(id) = params.find("id") {
            println!("Delete was sent successfully with id: {}", id);
        }

        Response::ok(String::from(""))
    });

    app.listen();
}
