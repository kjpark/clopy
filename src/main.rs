mod cli;
mod source;

use std::error;
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

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("200 Success. {:?}", response.status());
        }
        reqwest::StatusCode::NOT_FOUND => {
            println!("404 Not Found. {:?}", response.status());
            panic!("404 Not Found");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("401 Unauthorized {:?}", response.status());
            panic!("401 Unauthorized");
        }
        _ => {
            panic!("Invalid response status: {:?}", response.status());
        }
    };

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