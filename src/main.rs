mod cli;
mod source;

use std::error;
use std::process;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = cli::Cli::parse();
    let url = source::Source::from(&args.source).to_url();

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "reqwest")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?;

    handle_response(&url, response.status());

    if args.verbose {
        // breaks with gitlab
        // println!("Received {:?} bytes", response.content_length().unwrap());
        println!("{}", url);
        println!("{:?}", args);
        println!("{:?}", response.headers());
    }

    let bytes: Vec<u8> = response.bytes().await?.to_vec();
    unpack(bytes, args.destination)?;

    Ok(())
}

fn handle_response(url: &str, status_code: reqwest::StatusCode) {
    match status_code {
        reqwest::StatusCode::OK => {
            println!("Response: {}", status_code);
        }
        reqwest::StatusCode::NOT_FOUND => {
            eprintln!("Error: {}", status_code);
            // todo: print help 404
            process::exit(1);
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            eprintln!("Error: {}", status_code);
            // todo: print help for auth
            process::exit(1);
        }
        _ => {
            eprintln!("Error: {}", status_code);
            process::exit(1);
        }
    }
}

fn unpack(bytes: Vec<u8>, destination: Option<String>) -> Result<(), Box<dyn error::Error>> {
    let destination = match destination {
        Some(destination) => destination,
        None => String::from("."),
    };

    let tar = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(destination)?;
    Ok(())
}
