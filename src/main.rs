use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::io::prelude::*;
use std::net::SocketAddr;

async fn app(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let mut file = std::fs::File::open("./index.html").expect("failed to find index.html");
            let mut layout = String::new();
            file.read_to_string(&mut layout)
                .expect("failed to read file");

            Ok(Response::new(Body::from(layout)))
        }
        (&Method::GET, "/about") => {
            let mut file = std::fs::File::open("./about.html").expect("failed to find about.html");
            let mut layout = String::new();
            file.read_to_string(&mut layout)
                .expect("failed to read file");

            Ok(Response::new(Body::from(layout)))
        }
        (&Method::GET, "/tools") => {
            let mut file = std::fs::File::open("./tools.html").expect("failed to find tools.html");
            let mut layout = String::new();
            file.read_to_string(&mut layout)
                .expect("failed to read file");

            Ok(Response::new(Body::from(layout)))
        }
        (&Method::GET, "/ip") => {
            let mut ip = "0.0.0.0".to_string();
            let ip_header = req.headers().get("X-Forwarded-For");

            if let Some(header_value) = ip_header {
                let header_value = header_value.clone();
                ip = String::from(header_value.to_str().unwrap());
            }

            Ok(Response::new(Body::from(format!("{}\n", ip))))
        }
        (&Method::GET, "/u32") => {
            let params: std::collections::HashMap<String, String> = req
                .uri()
                .query()
                .map(|v| {
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(std::collections::HashMap::new);

            if let Some(num) = params.get("q") {
                match notryanb_dev::parse_num(num) {
                    Ok(answer) => {
                        let msg = format!("Hex: {answer:#x}\nOctal: {answer:#o}\nBinary: {answer:#b}\nDecimal: {answer}");
                        Ok(Response::new(Body::from(msg.to_string())))
                    },
                    Err(err) => Ok(Response::new(Body::from(err.to_string())))
                }
            } else {
                let msg = "Format a 32bit unsigned int to and from hex/octal/binary/decimal\nUsage: add query paramter, 'q'\n
examples:\n\t?q=0xc00ffee\n\t?q=0o666\n\t?q=0b01001011\n\t?q=8675309";
                Ok(Response::new(Body::from(msg)))
            }

        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_service =
        make_service_fn(move |_conn| async { Ok::<_, hyper::Error>(service_fn(app)) });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on https://{}", addr);
    server.await?;

    Ok(())
}
