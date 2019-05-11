use actix_web::{server, HttpRequest, Responder, App, Path};

use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", to)
}

fn hello_name(to: Path<HelloPath>) -> impl Responder {
    format!("Hello {}!", &to.name)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(hello))
            .resource("/{name}", |r| r.with(hello_name))
    }).bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}
