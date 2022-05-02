// https://docs.github.com/en/rest/repos/contents#download-a-repository-archive-tar
// GET /repos/{owner}/{repo}/tarball/{ref}
// https://api.github.com/repos/{}/{}/tarball/{}

// https://docs.gitlab.com/ee/api/repositories.html#get-file-archive
// GET https://gitlab.example.com/api/v4/projects/:id/repository/archive[.format]
// https://gitlab.com/api/v4/projects/{}%2F{}/repository/archive?sha=<commit_sha>&path=<path>

pub struct Source {
    host: Host,
    owner: String,
    repo: String,
    tag: Option<String>,
}

enum Host {
    Github,
    Gitlab, // self hosted?
            // Bitbucket,
}

pub fn parse_source(source: &str) -> Source {
    let parts: Vec<&str> = source
        // BUG: if tag has `/` in it, tag will be split
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
        }
        // host/owner/repo[:tag]
        3 => {
            source.host = match parts[0] {
                "github.com" => Host::Github,
                "gitlab.com" => Host::Gitlab,
                _ => panic!("Unsupported host"),
            };
            source.owner = parts[1].to_string();
        }
        _ => {
            panic!("Invalid source format");
        }
    };

    // check last arg for tag, set the repo [and tag]
    let last_part = parts[parts.len() - 1];
    match last_part.find(':') {
        Some(index) => {
            source.repo = last_part[..index].to_string();
            source.tag = Some(last_part[index + 1..].to_string());
        }
        None => {
            source.repo = last_part.to_string();
        }
    }

    source
}

// impl Source.gen_url(); instead
pub fn gen_url(source: &Source) -> String {
    let url = match source.host {
        Host::Github => {
            format!(
                "https://api.github.com/repos/{}/{}/tarball/{}",
                source.owner,
                source.repo,
                source.tag.clone().unwrap_or_else(|| String::from(""))
            )
        }
        Host::Gitlab => {
            let tag = match source.tag.clone() {
                Some(tag) => format!("?sha={}", tag),
                None => String::from(""),
            };
            format!(
                "https://gitlab.com/api/v4/projects/{}%2F{}/repository/archive{}",
                source.owner, source.repo, tag
            )
        }
    };

    url
}