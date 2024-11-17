use std::net::{Ipv4Addr, Ipv6Addr};
use regex::Regex;

/*
    Validate URL:
*/
pub fn validate_ip_and_port(url: &str) -> Result<(), String> {
    // Use regex to extract the host and port (optional) from the URL:
    /*
        - `https?://` matches both "http://" and "https://"
        - `(\[.*?\]|[^:/]+)` matches the host portion of the URL
        - `(?::(\d+))?` matches the port portion of the URL
    */
    let re = Regex::new(r"https?://(\[.*?\]|[^:/]+)(?::(\d+))?").unwrap();
    if let Some(caps) = re.captures(url) {
        let host = &caps[1]; // strips the protocol and port

        let ipv6_regex = Regex::new(r"^\[([a-fA-F0-9:.%]+)\]$").unwrap();
        let ipv4_regex = Regex::new(r"^\d{1,3}(\.\d{1,3}){3}$").unwrap();
        
        // Validate as an IPv4 or IPv6 address:
        if ipv6_regex.is_match(host) {
            // Treat it as an IPv6 address:
            let ipv6 = &host[1..host.len() - 1];
            if ipv6.parse::<Ipv6Addr>().is_err() {
                return Err("The URL contains an invalid IPv6 address.".to_string());
            }
        } else if ipv4_regex.is_match(host) {
            // Treat it as an IPv4 address:
            if host.parse::<Ipv4Addr>().is_err() {
                return Err("The URL contains an invalid IPv4 address.".to_string());
            }
        }

        // Validate the port number (if present):
        if let Some(port_str) = caps.get(2) {
            // Try to parse the port string to a u16 (0 ~ 65535):
            if let Err(_) = port_str.as_str().parse::<u16>() {
                return Err("The URL contains an invalid port number.".to_string());
            }
        }
    }
    Ok(())
}