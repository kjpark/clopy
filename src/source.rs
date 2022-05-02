// https://docs.github.com/en/rest/repos/contents#download-a-repository-archive-tar
// GET /repos/{owner}/{repo}/tarball/{ref}
// https://api.github.com/repos/{}/{}/tarball/{}

// https://docs.gitlab.com/ee/api/repositories.html#get-file-archive
// GET https://gitlab.example.com/api/v4/projects/:id/repository/archive[.format]
// https://gitlab.com/api/v4/projects/{}%2F{}/repository/archive?sha=<commit_sha>&path=<path>

use regex::Regex;

pub struct Source {
    host: Host,
    owner: String,
    repo: String,
    tag: Option<String>,
}

// Source::from("kjpark/clopy:dev").to_url();
impl Source {
    pub fn from(source: &str) -> Source {
        parse_source(source)
    }
    pub fn to_url(&self) -> String {
        gen_url(&self)
    }
}

enum Host {
    Github,
    Gitlab, // self hosted?
            // Bitbucket,
}

fn parse_source(input: &str) -> Source {
    let mut source = Source {
        host: Host::Github, // default to github
        owner: String::new(),
        repo: String::new(),
        tag: None,
    };

    // captures:
    // [host]
    // owner
    // repo_only | (repo, tag)
    let re = Regex::new(
        r"(?x)
        (?P<host>(github|gitlab)\.com/)?
        (?P<owner>.+)
        /
        ((?P<repo_only>[^:]+$)|(?P<repo>.+):(?P<tag>.+$))
        ",
    )
    .unwrap();

    let captures = re.captures(input).unwrap();

    // temp
    println!("{:#?}", captures);

    // temp return nothing useful
    source
}

fn gen_url(source: &Source) -> String {
    let url = match source.host {
        Host::Github => {
            let tag = match source.tag {
                Some(ref tag) => String::from(tag),
                None => String::new(),
            };
            format!(
                "https://api.github.com/repos/{}/{}/tarball/{}",
                source.owner, source.repo, tag
            )
        }
        Host::Gitlab => {
            let tag = match source.tag {
                Some(ref tag) => format!("?sha={}", tag),
                None => String::new(),
            };
            format!(
                "https://gitlab.com/api/v4/projects/{}%2F{}/repository/archive{}",
                source.owner, source.repo, tag
            )
        }
    };

    url
}

// test code to generate regex
// todo: create real unit test

// fn main() {
//     let re = Regex::new(
//         r"(?x)
//         (?P<host>(github|gitlab)\.com/)?
//         (?P<owner>.+)
//         /
//         ((?P<repo_only>[^:]+$)|(?P<repo>.+):(?P<tag>.+$))
//         ",
//     )
//     .unwrap();

//     let test_cases = vec![
//         "github.com/kjpark/clopy:dev",
//         "gitlab.com/kjpark/clopy",
//         "kjpark/clopy:dev",
//         "kjpark/clopy",
//     ];

//     for s in test_cases {
//         println!("{:#?}", re.captures(s).unwrap());
//     }
// }
