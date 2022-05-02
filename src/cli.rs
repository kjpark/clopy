use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// "[host/]owner/repo[:branch|:tag|:commit]"
    pub source: String,

    /// "output/path"
    pub destination: Option<String>,

    #[clap(short, long)]
    /// verbose output?
    pub verbose: bool,
}
