#![allow(dead_code)]
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    // Set Default Path to Public in an environment variable, using env macro and Cargo's
    // metadata

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    // Environment Variables for Public Path to serve HTML files
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public Path: {}", public_path);

    // Server Address. From string literal to String.
    let addr_string = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());
    println!("Public Path: {}", addr_string);

    // Startup the server.
    let server = Server::new(addr_string);
    server.run(WebsiteHandler::new(public_path));
}
