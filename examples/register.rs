#[macro_use]
extern crate serde_json;
extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Request, Response, Server};

fn echo(req: Request<Body>) -> Response<Body> {
    match (req.method(), req.uri().path()) {
        // Simply echo the body back to the client.
        (&Method::POST, "/write") => {
            let body = json!({"result":"OK"});
            let body = serde_json::to_string(&body).unwrap();
            println!("Response body: {}", &body);
            Response::new(Body::from(body))
        }

        // The 404 Not Found route...
        _ => {
            println!("StatusCode::NOT_FOUND");
            Response::new(Body::empty())
        }
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 12345).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
