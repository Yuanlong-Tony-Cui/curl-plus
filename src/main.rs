use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::BTreeMap;
use std::process;
use structopt::StructOpt;
use std::net::{Ipv4Addr, Ipv6Addr};
use regex::Regex;

/// Define CLI arguments
#[derive(StructOpt, Debug)]
struct Cli {
    url: String,
    #[structopt(short = "X", long)]
    method: Option<String>,
    #[structopt(short = "d", long)]
    data: Option<String>,
    #[structopt(long = "json")]
    json_data: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    // Check for a valid protocol in the URL before parsing
    if !args.url.starts_with("http://") && !args.url.starts_with("https://") {
        print_error(&args.url, "The URL does not have a valid base protocol.");
        process::exit(1);
    }

    // Perform IP and port validation before parsing the URL
    if let Err(err) = validate_ip_and_port(&args.url) {
        print_error(&args.url, &err);
        process::exit(1);
    }

    // Now parse the URL using the `url` crate
    let url = match url::Url::parse(&args.url) {
        Ok(u) => u,
        Err(_) => {
            print_error(&args.url, "The URL could not be parsed.");
            process::exit(1);
        }
    };

    let client = Client::new();

    match args.method.as_deref() {
        Some("POST") => {
            if let Some(json_data) = &args.json_data {
                handle_post_json(&client, &url, json_data).await;
            } else if let Some(data) = &args.data {
                handle_post_data(&client, &url, data).await;
            } else {
                eprintln!("Error: POST method requires data or JSON data.");
                process::exit(1);
            }
        }
        _ => handle_get(&client, &url).await,
    }
}

// Function to validate IP address and port in the URL string
fn validate_ip_and_port(url: &str) -> Result<(), String> {
    // Use regex to extract the host (IP) and optional port from the URL string
    let re = Regex::new(r"https?://(\[.*?\]|[^:/]+)(?::(\d+))?").unwrap();
    if let Some(caps) = re.captures(url) {
        let host = &caps[1];
        
        // Check if the host is a valid IPv4 or IPv6 address
        if host.starts_with('[') && host.ends_with(']') {
            // This is an IPv6 address; strip the brackets and validate
            let ipv6 = &host[1..host.len() - 1];
            if ipv6.parse::<Ipv6Addr>().is_err() {
                return Err("The URL contains an invalid IPv6 address.".to_string());
            }
        } else if host.parse::<Ipv4Addr>().is_err() {
            return Err("The URL contains an invalid IPv4 address.".to_string());
        }

        // Check if the port number (if present) is within the valid range
        if let Some(port_str) = caps.get(2) {
            // Try to parse the port string to a u16 and handle any parsing errors
            if let Err(_) = port_str.as_str().parse::<u16>() {
                return Err("The URL contains an invalid port number.".to_string());
            }
        }
    }
    Ok(())
}

// Helper function to print error message in the specified format
fn print_error(url: &str, message: &str) {
    println!("Requesting URL: {}", url);
    println!("Method: GET");
    println!("Error: {}", message);
}

// Handle GET request
async fn handle_get(client: &Client, url: &url::Url) {
    println!("Requesting URL: {}", url);
    println!("Method: GET");

    match client.get(url.as_str()).send().await {
        Ok(response) => print_response(response).await,
        Err(err) => handle_request_error(err),
    }
}

// Handle POST request with form data
async fn handle_post_data(client: &Client, url: &url::Url, data: &str) {
    println!("Requesting URL: {}", url);
    println!("Method: POST");
    println!("Data: {}", data);

    let form_data: Vec<(&str, &str)> = data
        .split('&')
        .map(|pair| {
            let mut kv = pair.split('=');
            (kv.next().unwrap_or(""), kv.next().unwrap_or(""))
        })
        .collect();

    match client.post(url.as_str()).form(&form_data).send().await {
        Ok(response) => print_response(response).await,
        Err(err) => handle_request_error(err),
    }
}

// Handle POST request with JSON data
async fn handle_post_json(client: &Client, url: &url::Url, json_data: &str) {
    println!("Requesting URL: {}", url);
    println!("Method: POST");
    println!("JSON: {}", json_data);

    let json: Value = match serde_json::from_str(json_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Invalid JSON: {}", e);
            process::exit(1);
        }
    };

    match client
        .post(url.as_str())
        .header("Content-Type", "application/json")
        .json(&json)
        .send()
        .await
    {
        Ok(response) => print_response(response).await,
        Err(err) => handle_request_error(err),
    }
}

// Print the response body
async fn print_response(response: Response) {
    if response.status() != reqwest::StatusCode::OK {
        eprintln!("Error: Request failed with status code: {}.", response.status());
        process::exit(1);
    }

    let body = response.text().await.unwrap_or_default();
    if let Ok(json) = serde_json::from_str::<Value>(&body) {
        let sorted_json: BTreeMap<_, _> = json.as_object().unwrap().clone().into_iter().collect();
        println!("Response body (JSON with sorted keys):");
        println!("{}", serde_json::to_string_pretty(&sorted_json).unwrap());
    } else {
        println!("Response body:");
        println!("{}", body);
    }
}

// Handle request errors
fn handle_request_error(err: reqwest::Error) {
    if err.is_connect() {
        eprintln!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
    } else {
        eprintln!("Error: Request failed. {}", err);
    }
    process::exit(1);
}
