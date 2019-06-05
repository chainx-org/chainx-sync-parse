use hyper::{rt::Future, service::service_fn_ok, Body, Method, Request, Response, Server};
use serde_json::json;
use structopt::StructOpt;

fn echo(req: Request<Body>) -> Response<Body> {
    let (parts, _body) = req.into_parts();

    match (parts.method, parts.uri.path()) {
        // Simply echo the body back to the client.
        (Method::POST, "/write") => {
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "register-server-test",
    author = "ChainX <https://chainx.org>",
    about = "For testing register service"
)]
struct Opt {
    /// Specify the ip address
    #[structopt(
        short = "i",
        long = "ip",
        default_value = "127.0.0.1",
        parse(from_str = "parse_ip_addr")
    )]
    ip: [u8; 4],
    /// Specify the port of register service
    #[structopt(short = "p", long = "port", default_value = "12345")]
    port: u16,
}

fn parse_ip_addr(ip_addr: &str) -> [u8; 4] {
    let ip_addr: Vec<u8> = ip_addr
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    let mut ip = [0u8; 4];
    ip.copy_from_slice(ip_addr.as_slice());
    ip
}

fn main() {
    let opt = Opt::from_args();
    let addr = (opt.ip, opt.port).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
