use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{process::Command, str::FromStr};
use tiny_http::{Header, Method, Response, Server};

#[derive(Deserialize)]
struct CliCommand {
    command: String,
    args: Vec<String>,
}

#[derive(Serialize)]
struct CommandResult {
    stdout: String,
    stderr: String,
    status: i32,
}

#[derive(Parser)]
#[command(author, version, about = "CLI to REST API converter")]
struct Args {
    #[arg(short, long, default_value = "[::]")]
    bind: String,
    #[arg(short, long, default_value = "8000")]
    port: u16,
}

fn main() {
    let args = Args::parse();
    let addr = format!("{}:{}", args.bind, args.port);
    let server = Server::http(&addr).unwrap();
    println!("Starting CLI2REST server on http://{}", addr);

    for mut request in server.incoming_requests() {
        if request.method() == &Method::Post && request.url() == "/execute" {
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).unwrap();

            if let Ok(command) = serde_json::from_str::<CliCommand>(&content) {
                let output = Command::new(&command.command)
                    .args(&command.args)
                    .output()
                    .unwrap_or_else(|e| panic!("Failed to execute command: {}", e));

                let result = CommandResult {
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    status: output.status.code().unwrap_or(-1),
                };

                let response = Response::from_string(serde_json::to_string(&result).unwrap())
                    .with_header(Header::from_str("Content-Type: application/json").unwrap());
                    
                request.respond(response).unwrap();
            } else {
                let response = Response::from_string("Invalid JSON")
                    .with_status_code(400);
                request.respond(response).unwrap();
            }
        } else {
            let response = Response::from_string("Not Found")
                .with_status_code(404);
            request.respond(response).unwrap();
        }
    }
}
