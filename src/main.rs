use std::fs::{self, File};
use std::io::BufReader;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use cv_website::render::render;
use cv_website::resume::Resume;
use tiny_http::{Header, Response, Server};

#[derive(Parser)]
#[command(about = "Generate the CV website, or serve it locally for testing")]
struct Cli {
    #[arg(long, default_value_t = String::from("templates"))]
    template_directory: String,

    #[arg(short, long, default_value = "./resume.yml")]
    input: PathBuf,

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

fn serve(ip: IpAddr, port: u16, content: &str) {
    let addr = SocketAddr::new(ip, port);
    let server = Server::http(addr).unwrap();
    eprintln!("Serving on http://{addr}");
    for request in server.incoming_requests() {
        let url = request.url();
        let response = if url == "/" || url == "/index.html" {
            Response::from_string(content).with_header(header("text/html; charset=utf-8"))
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

fn main() -> Result<()> {
    let parsed = Cli::parse();
    let input_file = File::open(parsed.input).context("Failed to open input file")?;
    let reader = BufReader::new(input_file);
    let resume: Resume = yaml_serde::from_reader(reader).context("Failed to parse input YAML")?;
    let rendered = render(&format!("{}/*.html", parsed.template_directory), &resume)?;
    match parsed.command {
        Command::Generate { output } => match output {
            Some(path) => fs::write(path, rendered).unwrap(),
            None => print!("{}", rendered),
        },
        Command::Serve { ip, port } => serve(ip, port, &rendered),
    }
    Ok(())
}
