use std::net::Ipv4Addr;

use hyper::{service::{service_fn, make_service_fn}, Body, Method, Request, Response, Server};
use serde_json::json;
use structopt::StructOpt;

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let (parts, _body) = req.into_parts();

    match (parts.method, parts.uri.path()) {
        // Simply echo the body back to the client.
        (Method::POST, "/write") => {
            let body = json!({"result":"OK"});
            let body = serde_json::to_string(&body).unwrap();
            println!("Response body: {}", &body);
            Ok(Response::new(Body::from(body)))
        }

        // The 404 Not Found route...
        _ => {
            println!("StatusCode::NOT_FOUND");
            Ok(Response::new(Body::empty()))
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "register-server-test",
    author = "ChainX <https://chainx.org>",
    about = "For testing register service"
)]
struct Opt {
    /// Specify the ip address
    #[structopt(short = "i", long = "ip", default_value = "127.0.0.1")]
    ip: Ipv4Addr,
    /// Specify the port of register service
    #[structopt(short = "p", long = "port", default_value = "12345")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let addr = (opt.ip, opt.port).into();

    let serve_fut = Server::bind(&addr)
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(echo))
        }));
    println!("Listening on http://{}", addr);

    if let Err(err) = serve_fut.await {
        eprintln!("server error: {}", err);
    }
}
