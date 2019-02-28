use clap::{App, Arg};
use hyper::{rt::Future, service::service_fn_ok, Body, Method, Request, Response, Server};
use serde_json::json;

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

fn config_ip_port() -> ([u8; 4], u16) {
    let matches = App::new("register-server-test")
        .version("1.0")
        .author("ChainX <https://chainx.org>")
        .about("For testing register service")
        .arg(
            Arg::with_name("ip")
                .short("i")
                .long("ip")
                .value_name("IP")
                .help("Specify the ip address")
                .default_value("127.0.0.1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Specify the port of register service")
                .default_value("12345")
                .takes_value(true),
        )
        .get_matches();

    let ip_addr = matches.value_of("ip").unwrap_or("127.0.0.1");
    let ip_addr: Vec<u8> = ip_addr
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    let mut ip = [0u8; 4];
    ip.copy_from_slice(ip_addr.as_slice());
    let port = matches.value_of("port").unwrap_or("12345");
    let port = port.parse::<u16>().unwrap();
    (ip, port)
}

fn main() {
    let (ip, port) = config_ip_port();
    let addr = (ip, port).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
