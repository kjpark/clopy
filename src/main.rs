// https://docs.github.com/en/rest/reference/repos#download-a-repository-archive-tar
// GET /repos/{owner}/{repo}/tarball/{ref}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let cli = Args::parse();
    println!("{:?}", cli);
    // validate & parse later with regex
    // let url = format!("https://api.github.com/repos/{}/tarball/main", cli.source);

    let source = parse_source(&cli.source);
    let url = format!("https://api.{}/repos/{}/{}/tarball/{}",
                      source.host,
                      source.owner,
                      source.repo,
                      source.tag);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "reqwest")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("200 Success. {:?}", response.status());
        },
        reqwest::StatusCode::NOT_FOUND => {
            println!("404 Not Found. {:?}", response.status());
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("401 Unauthorized {:?}", response.status());
        },
        _ => {
            panic!("Uncaught error, please file bug report <3");
        },
    };

    if cli.verbose {
        println!("Received {:?} bytes", response.content_length().unwrap());
        println!("{:?}", response.headers());
    }
    
    let destination = match cli.destination {
        Some(destination) => destination,
        None => String::from("."),
    };

    let bytes: Vec<u8> = response.bytes().await?.to_vec();
    let tar = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(destination).unwrap_or_else(|e| {
        panic!("Error unpacking tarball: {}", e);
    });

    Ok(())
}

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // todo owner/repo[:branch|:tag]
    /// "owner/repo"
    source: String,

    /// "output/path"
    destination: Option<String>,

    #[clap(short, long)]
    /// verbose output?
    verbose: bool,
}

// enum Host {
//     Github,
//     Gitlab, // self hosted?
//     // Bitbucket,
// }

struct Source {
    // host: Host,
    host: String,
    owner: String,
    repo: String,
    tag: String,
}

fn parse_source(source: &str) -> Source {
    let parts: Vec<&str> = source.split('/').collect();
    Source {
        // host: match parts[0] {
        //     "github.com" => Host::Github,
        ///////"github" => "https://api.{}/repos/{}/{}/tarball/{}"
        //     "gitlab.com" => Host::Gitlab,
        //     // "bitbucket.org" => Host::Bitbucket,
        //     _ => panic!("Unsupported host"),
        // },
        host: parts[0].to_string(),
        owner: parts[1].to_string(),
        repo: parts[2].to_string(),
        tag: parts[3].to_string(),
    }
}