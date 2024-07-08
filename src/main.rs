use clap::Parser;
use scrapper::{Analiser, VerboseLevel};
use url::Url;

/// Simple tool to explore and trace the pathways within any website.
/// Creating a comprehensive "path web"
#[derive(Parser, Debug)]
#[command(version = "1.0.0v", about)]
struct Args {
    /// First URL to be scanned. Needs to include "http" / "https".
    #[arg(short, long, value_parser = validate_initial_url)]
    url: String,

    /// Number of the max depth of the Breadth First search.
    #[arg(short, long, default_value_t = 3)]
    depth: usize,

    /// Prints every new URL relation when it founds one.
    /// [0]: Log disabled on the console.
    /// [1]: Only successfull atempts are printed on the console
    /// [2]: All the atempts are printed on the console
    #[arg(short, long, default_value_t = 1, value_parser = validate_verbose)]
    verbose: u8,

    /// Number of the max URLs relation to be found.
    #[arg(long, default_value_t = 10000)]
    max_urls: usize,

    /// Every new URL Discover needs to match this str. Here, it could be
    /// passed a domain, to search only for paths within the main website.
    #[arg(long, default_value_t = String::from(""))]
    match_str: String,

    /// Ignore all URLs containing this strings, splitted by `,`
    #[arg(short, long, default_value_t = String::from(""))]
    ignore: String,
}

fn validate_initial_url(url: &str) -> Result<String, String> {
    Url::parse(url).map(|_| url.to_string()).map_err(|_| {
        format!(
            "'{}' is not a valid URL. Make sure that it starts with \"http://\" or \"http://\"",
            url
        )
    })
}

fn validate_verbose(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(val) if val <= 2 => Ok(val),
        Ok(_) => Err(String::from("Verbose must be between 0 and 2")),
        Err(_) => Err(String::from("Verbose must be a number")),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let ignore_str: Vec<&str> = args
        .ignore
        .split(',')
        .map(|s| s.trim())
        .collect();

    let mut analiser = Analiser::new(
        &args.url,
        &args.match_str,
        args.depth,
        args.max_urls,
        VerboseLevel::from_u8(args.verbose),
        ignore_str,
    );
    analiser.start().await;
}
