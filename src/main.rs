mod crawlers;

extern crate regex;

use regex::Regex;

fn main() {
    let ip_port_pattern_global = Regex::new(r"(?P<ip>(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?))(?=.*?(?:(?:(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?))|(?P<port>\d{2,5})))")
        .expect("Failed to create regex");

    let text = "192.168.1.1:8080";

    if let Some(captures) = ip_port_pattern_global.captures(text) {
        if let Some(ip) = captures.name("ip") {
            println!("IP: {}", ip.as_str());
        }

        if let Some(port) = captures.name("port") {
            println!("Port: {}", port.as_str());
        }
    }
}
