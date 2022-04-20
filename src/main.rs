// https://docs.github.com/en/rest/reference/repos#download-a-repository-archive-tar
// GET /repos/{owner}/{repo}/tarball/{ref}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/kjpark/clopy/tarball/main")
        .header("User-Agent", "reqwest")
        .header("accept", "application/vnd.github.v3+json")
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

    Ok(())
}