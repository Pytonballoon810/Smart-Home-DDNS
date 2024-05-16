use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use std::time;

use http_body_util::{Full, Empty};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};

use dotenv;

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let query = req.uri().query().unwrap_or("");
    let params: Vec<&str> = query.split('&').collect();

    let mut username = "";
    let mut pwd = "";
    let mut domain = "";
    let mut ip = "";

    for param in params {
        let pair: Vec<&str> = param.split('=').collect();
        if pair[0] == "username" {
            username = pair[1];
        } else if pair[0] == "pwd" {
            pwd = pair[1];
        } else if pair[0] == "domain" {
            domain = pair[1];
        } else if pair[0] == "ip" {
            ip = pair[1];
        } else {
            println!("Unknown parameter: {}", pair[0]);
        }
    }

    if username != env::var("USERNAME").unwrap_or(String::from(""))
        || pwd != env::var("PASSWD").unwrap_or(String::from(""))
    {
        return Ok(Response::new(Full::new(Bytes::from(
            "Invalid username or password",
        ))));
    } else if domain != env::var("DOMAIN").unwrap() {
        return Ok(Response::new(Full::new(Bytes::from("Invalid domain"))));
    } else {
        println!("Request received. New IP: {}", ip);
        println!("Updating DNS record...");



        return Ok(Response::new(Full::new(Bytes::from(format!(
            "Received. Updating DNS record... at {:?}",
            time::SystemTime::now()
        )))));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    let current_dir: std::path::PathBuf =
        env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);
    let env_file: std::path::PathBuf = current_dir.join(".env");
    if env_file.exists() {
        println!(".env File exists, using dotenv as environment variable loader");
    } else {
        println!(".env File does not exist, using system environment variables directly");
    }

    // Client Side
    let uri = "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{dns_record_id}".parse::<hyper::Uri>()?;

    let host = uri.host().expect("Uri has no host");
    let port = uri.port_u16().unwrap_or(80);

    let client_addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(client_addr).await?;

    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection failed: {:?}", err);
        }
    });

    // Server Side
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let listener = TcpListener::bind(server_addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("server connection error: {}", e);
            }
        });
    }
}
