mod cli;
mod source;

use std::error;
use std::process;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = cli::Cli::parse();
    let source = source::Source::from(&args.source);

    let url = source.to_url();
    let header_map = create_headers(&source);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .headers(header_map)
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

fn create_headers(source: &source::Source) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("clopy"),
    );

    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    // if let Some(token) = source.token() {
    //     headers.insert(
    //         reqwest::header::AUTHORIZATION,
    //         reqwest::header::HeaderValue::from_str(&format!("token {}", token)).unwrap(),
    //     );
    // }

    headers
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
