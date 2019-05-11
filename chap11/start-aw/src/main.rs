use actix_web::{server, HttpRequest, Responder, App, Path, State};

use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello(req: HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", to)
}

fn hello_name(to: Path<HelloPath>) -> impl Responder {
    format!("Hello {}!", &to.name)
}

#[derive(Deserialize)]
struct MyApp {
    server_name: String,
}

fn hello_with_state(app: State<MyApp>) -> impl Responder {
    format!("Hello from {}!", &app.server_name)
}

fn main() {
    server::new(|| {
        App::with_state(MyApp {
            server_name: "server with state".into(),
        })
            .resource("/info", |r| r.with(hello_with_state))
            .resource("/{name}", |r| r.with(hello_name))
    }).bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}
