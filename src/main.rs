#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() -> Result<(), String> {
    // Get default path for `public` folder from environment
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    println!("{}", public_path);

    // Create a address for TCP bind
    let server = Server::new_full_address("127.0.0.1:8080")?;

    // Run TCP server
    server.run(WebsiteHandler::new(public_path));

    Ok(())
}
