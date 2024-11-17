mod cli;
mod validators;
mod handlers;

use cli::Cli;
use validators::validate_ip_and_port;
use handlers::{handle_get, handle_post_data, handle_post_json};
use reqwest::Client;
use std::process;

fn print_error(url: &str, message: &str) {
    println!("Requesting URL: {}", url);
    println!("Method: GET");
    println!("Error: {}", message);
}

#[tokio::main]
async fn main() {
    // Collect input from CLI:
    let args = Cli::parse_args();

    // Validate the base protocol:
    if !args.url.starts_with("http://") && !args.url.starts_with("https://") {
        print_error(&args.url, "The URL does not have a valid base protocol.");
        process::exit(1);
    }

    // Validate the URL format:
    if let Err(err) = validate_ip_and_port(&args.url) {
        print_error(&args.url, &err);
        process::exit(1);
    }

    // Parse the URL:
    let url = match url::Url::parse(&args.url) {
        Ok(u) => u,
        Err(_) => {
            print_error(&args.url, "The URL could not be parsed.");
            process::exit(1);
        }
    };

    let client = Client::new();

    if let Some(json_data) = &args.json_data {
        // If the `--json` flag is used, perform a POST with JSON data:
        handle_post_json(&client, &url, json_data).await;
    } else if args.method.as_deref() == Some("POST") {
        // If `-X POST` is used, perform a POST request with key-value-pair data:
        if let Some(data) = &args.data {
            handle_post_data(&client, &url, data).await;
        } else {
            eprintln!("Error: POST method requires data.");
            process::exit(1);
        }
    } else {
        // Use GET by default:
        handle_get(&client, &url).await;
    }
}