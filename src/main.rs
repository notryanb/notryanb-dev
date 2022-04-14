use std::io::prelude::*;
use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
pub struct Context {
    content: String,
}

async fn app(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            // read in the base template layout to a buffer
            let mut file = std::fs::File::open("./layout.html")
                .expect("failed to find index.html");
            let mut layout = String::new();
            file.read_to_string(&mut layout)
                .expect("failed to read file");

            // Pass the template buffer into the templater
            let mut tt = TinyTemplate::new();
            tt.add_template("thingy", &layout).expect("failed to add template");
            let context = Context {
                content: "Test page content".into()
            };

            let rendered = tt.render("thingy", &context)
                .expect("failed to render template");

            Ok(Response::new(Body::from(rendered)))
        },
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let make_service = make_service_fn(|_conn| {
        async {  Ok::<_, hyper::Error>(service_fn(app)) }
    });

    let addr = SocketAddr::from(([127,0,0,1], 3333));
    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on https://{}", addr);
    server.await?;

    Ok(())
}
