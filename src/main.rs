use std::io::prelude::*;
use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

async fn app(req: Request<Body>, addr: SocketAddr) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let mut file = std::fs::File::open("./index.html")
                .expect("failed to find index.html");
            let mut layout = String::new();
            file.read_to_string(&mut layout)
                .expect("failed to read file");

            Ok(Response::new(Body::from(layout)))
        },
        (&Method::GET, "/about") => {
            let mut file = std::fs::File::open("./about.html")
                .expect("failed to find about.html");
            let mut layout = String::new();
            file.read_to_string(&mut layout)
                .expect("failed to read file");

            Ok(Response::new(Body::from(layout)))
        },
        (&Method::GET, "/tools") => {
            let mut file = std::fs::File::open("./tools.html")
                .expect("failed to find tools.html");
            let mut layout = String::new();
            file.read_to_string(&mut layout)
                .expect("failed to read file");

            Ok(Response::new(Body::from(layout)))
        },
        (&Method::GET, "/ip") => {
            let layout = addr.to_string();

            Ok(Response::new(Body::from(layout)))
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

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        async move {  
            let addr = addr.clone();
            Ok::<_, hyper::Error>(service_fn(move |req| app(req, addr))) 
        }
    });

    let addr = SocketAddr::from(([127,0,0,1], 3333));
    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on https://{}", addr);
    server.await?;

    Ok(())
}
