use std::convert::Infallible;
use std::env;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Mutex;
use std::time;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use dotenv;

use reqwest;

use serde_json::Value;

use lazy_static::lazy_static;

// declare global variables
lazy_static! {
    pub static ref LAST_UPDATED: Mutex<time::SystemTime> = Mutex::new(time::SystemTime::now());
    pub static ref LAST_IP: Mutex<String> = Mutex::new(String::from("0.0.0.0"));
}

// handle_request function to handle incoming requests from the client to the web page
async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let query = req.uri().query().unwrap_or("");
    if query == "" {
        return Ok(Response::new(Full::new(Bytes::from(format!(
            "No query parameters found. Last updated at {:?} with IP: {:?}",
            *LAST_UPDATED.lock().unwrap(),
            *LAST_IP.lock().unwrap()
        )))));
    }
    // get the query parameters from the request url (these must be supplied by the fritzbox.)
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

    // check if the username and password are correct
    if username != env::var("USERNAME").unwrap_or(String::from(""))
        || pwd != env::var("PASSWD").unwrap_or(String::from(""))
    {
        return Ok(Response::new(Full::new(Bytes::from(
            "Invalid username or password",
        ))));
    }
    // check if the domain is correct
    else if domain != env::var("DOMAIN").unwrap() {
        return Ok(Response::new(Full::new(Bytes::from("Invalid domain"))));
    } else {
        println!("Request received. New IP: {}", ip);
        println!("Updating DNS record...");

        _ = call_cloudflare_api(ip.parse().unwrap());

        return Ok(Response::new(Full::new(Bytes::from(format!(
            "Received. Updating DNS record at {:?}",
            time::SystemTime::now()
        )))));
    };
}

#[tokio::main]
async fn call_cloudflare_api(new_ip: Ipv4Addr) -> Result<(), Box<dyn std::error::Error>> {
    // get environment variables
    let zone_id = env::var("ZONE_ID").unwrap();
    // get the A record name from the environment variables, if not found, use the domain name
    let a_record_name = env::var("A_RECORD_NAME").unwrap_or(env::var("DOMAIN").unwrap());
    if a_record_name == env::var("DOMAIN").unwrap() {
        println!("A_RECORD_NAME matches with DOMAIN. Either because A_RECORD_NAME is not set or A_RECORD_NAME is set to DOMAIN.");
    };
    let api_key = env::var("API_KEY").unwrap();
    // get all DNS records from the zone corresponding to the zone_id
    let records_resp = reqwest::Client::new()
        .get(format!(
            "https://api.cloudflare.com/client/v4/zones/{:?}/dns_records",
            &zone_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await
        .unwrap()
        .text()
        .await;
    // get the record_id of the A record corresponding to the a_record_name from the records list
    let record_id = get_id_from_dns_record_list(&a_record_name, &records_resp.unwrap()).await?;
    // update the A record with the new IP
    let update_dns_record_resp = reqwest::Client::new()
        .patch(format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            &zone_id, &record_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .body(format!(
            "{{
            \"content\": \"{new_ip}\",
            \"name\": \"{a_record_name}\",
            \"proxied\": {:?},
            \"type\": \"A\",
            \"comment\": \"Updated by ddns-rust on {:?}\",
            \"tags\": [],
            \"ttl\": 1
        }}",
            env::var("PROXIED").unwrap_or(String::from("false")),
            time::SystemTime::now()
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await;
    print!("Response: {:?}", update_dns_record_resp.unwrap());
    // update global variables
    let mut last_ip = LAST_IP.lock().unwrap();
    *last_ip = new_ip.to_string();
    update_last_updated();

    Ok(())
}

fn update_last_updated() {
    let mut last_updated = LAST_UPDATED.lock().unwrap();
    *last_updated = time::SystemTime::now();
}

async fn get_id_from_dns_record_list(name: &str, response: &str) -> Result<String, Error> {
    let data: Value = serde_json::from_str(response)?;

    if let Some(array) = data.get("result") {
        for item in array.as_array().unwrap() {
            if item["name"] == name {
                return Ok(item["id"].as_str().unwrap().to_string());
            }
        }
    }
    Err(Error::new(ErrorKind::Other, "No record found"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // look for .env file in the current directory to use instead of system environment variables
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

    // web server setup
    let port = env::var("PORT").unwrap_or(String::from("12080"));
    let server_addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port.parse().unwrap(),
    );

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
