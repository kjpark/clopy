// https://docs.github.com/en/rest/reference/repos#download-a-repository-archive-tar
// GET /repos/{owner}/{repo}/tarball/{ref}
// https://api.github.com/repos/{}/{}/tarball/{}

// https://docs.gitlab.com/ee/api/repositories.html#get-file-archive
// GET https://gitlab.example.com/api/v4/projects/:id/repository/archive[.format]
// https://gitlab.com/api/v4/projects/{}%2F{}/repository/archive?sha=<commit_sha>&path=<path>

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let cli = Args::parse();
    println!("{:?}", cli);

    let url = gen_url(&parse_source(&cli.source));
    println!("{}", url);

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
        // breaks with gitlab
        // println!("Received {:?} bytes", response.content_length().unwrap());
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

enum Host {
    Github,
    Gitlab, // self hosted?
    // Bitbucket,
}

struct Source {
    host: Host,
    // host: String,
    owner: String,
    repo: String,
    tag: Option<String>,
}

// args seperated by /
// strip empty
// match args.count() {
//     2 => owner/repo[:tag]
//     3 => host/owner/repo[:tag]
//     _ => panic!("no")
// }
fn parse_source(source: &str) -> Source {
    let parts: Vec<&str> = source
        .split('/')
        .filter(|&x| !x.is_empty())
        .collect();

    println!("{:?}", parts);

    let mut source = Source { 
        host: Host::Github, // default to github
        owner: String::from(""),
        repo: String::from(""),
        tag: None,
    };

    match parts.len() {
        // owner/repo[:tag]
        2 => {
            source.owner = parts[0].to_string();
            source.repo = parts[1].to_string();
            source.tag = None;
        },
        // host/owner/repo[:tag]
        3 => {
            source.host = match parts[0] {
                "github.com" => Host::Github,
                "gitlab.com" => Host::Gitlab,
                _ => panic!("Unsupported host"),
            };
            source.owner = parts[1].to_string();
            source.repo = parts[2].to_string();
            source.tag = None;
        },
        _ => {
            panic!("Invalid source format");
        }
    };

    source
}

// impl Source.gen_url(); instead
fn gen_url(source: &Source) -> String {
    let url = match source.host {
        Host::Github => {
            format!(
                "https://api.github.com/repos/{}/{}/tarball", // /{}",
                source.owner, source.repo // , source.tag
            )
        },
        Host::Gitlab => {
            format!(
                "https://gitlab.com/api/v4/projects/{}%2F{}/repository/archive", // ?sha={}",
                source.owner, source.repo // , source.tag
            )
        },
    };

    url
}