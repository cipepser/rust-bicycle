use actix_web::{server, App, fs};

fn main() {
    server::new(|| {
        App::new().handler(
            "/",
            fs::StaticFiles::new(".").unwrap(),
        )
    })
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}