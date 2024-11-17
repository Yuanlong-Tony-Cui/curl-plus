use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::BTreeMap;
use std::process;

// Handle GET request:
pub async fn handle_get(client: &Client, url: &url::Url) {
    println!("Requesting URL: {}", url);
    println!("Method: GET");

    match client.get(url.as_str()).send().await {
        Ok(response) => print_response(response).await,
        Err(err) => handle_request_error(err),
    }
}

// Handle POST request with form data:
pub async fn handle_post_data(client: &Client, url: &url::Url, data: &str) {
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

// Handle POST request with JSON data:
pub async fn handle_post_json(client: &Client, url: &url::Url, json_data: &str) {
    println!("Requesting URL: {}", url);
    println!("Method: POST");
    println!("JSON: {}", json_data);

    // Validate the JSON data using `serde_json`:
    let json: Value = serde_json::from_str(
        json_data
    ).expect("Invalid JSON"); // uses `expect` to print the panic output

    // Send the POST request with the JSON data:
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

// Format and print the response body:
async fn print_response(response: Response) {
    // Print status code on failed requests:
    if !response.status().is_success() {
        eprintln!("Error: Request failed with status code: {}.", response.status().as_u16());
        process::exit(1);
    }

    let body = response.text().await.unwrap_or_default();
    if let Ok(json) = serde_json::from_str::<Value>(&body) {
        // If the response body can be parsed as JSON:
        let sorted_json: BTreeMap<_, _> = json.as_object().unwrap().clone().into_iter().collect();
        println!("Response body (JSON with sorted keys):");
        println!("{}", serde_json::to_string_pretty(&sorted_json).unwrap());
    } else {
        // If the response body cannot be parsed as JSON:
        println!("Response body:");
        println!("{}", body);
    }
}

// Handle request errors:
fn handle_request_error(err: reqwest::Error) {
    if err.is_connect() {
        eprintln!("Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
    } else {
        eprintln!("Error: Request failed. {}", err);
    }
    process::exit(1);
}