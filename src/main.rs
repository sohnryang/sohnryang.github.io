use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tiny_http::{Header, Response, Server};

#[derive(Parser)]
#[command(about = "Generate the CV website, or serve it locally for testing")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate the page, writing to stdout or a file
    Generate {
        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Serve the generated page locally for testing
    Serve {
        /// IP address to bind to
        #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
        ip: IpAddr,
        /// Port to listen on
        #[arg(short, long, default_value_t = 8000)]
        port: u16,
    },
}

fn render() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Hello world</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <h1>Hello world</h1>
</body>
</html>
"#
}

fn content_type(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css",
        Some("js") => "text/javascript",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn header(content_type: &str) -> Header {
    Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap()
}

fn serve(ip: IpAddr, port: u16) {
    let addr = SocketAddr::new(ip, port);
    let server = Server::http(addr).unwrap();
    eprintln!("Serving on http://{addr}");
    for request in server.incoming_requests() {
        let url = request.url();
        let response = if url == "/" || url == "/index.html" {
            Response::from_string(render()).with_header(header("text/html; charset=utf-8"))
        } else if url.contains("..") {
            Response::from_string("Not found").with_status_code(404)
        } else {
            let path = format!("static/{}", url.trim_start_matches('/'));
            match fs::read(&path) {
                Ok(bytes) => Response::from_data(bytes).with_header(header(content_type(&path))),
                Err(_) => Response::from_string("Not found").with_status_code(404),
            }
        };
        eprintln!(
            "{} {} {} -> {}",
            chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            request.method(),
            request.url(),
            response.status_code().0
        );
        let _ = request.respond(response);
    }
}

fn main() {
    match Cli::parse().command {
        Command::Generate { output } => match output {
            Some(path) => fs::write(path, render()).unwrap(),
            None => print!("{}", render()),
        },
        Command::Serve { ip, port } => serve(ip, port),
    }
}
