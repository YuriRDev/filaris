use clap::Parser;
use scrapper::{Analiser, VerboseLevel};
use url::Url;

/// A web scraper tool to explore and trace pathways within websites.
#[derive(Parser, Debug)]
#[command(version = "1.0.0")]
struct Args {
    /// The initial URL to start scanning from. Ensure it starts with 'http' or 'https'.
    #[arg(short, long, value_parser = validate_initial_url)]
    url: String,

    /// Defines how deep the BFS should go.
    #[arg(short, long, default_value_t = 3)]
    depth: usize,

    /// Sets the verbosity level for logging URL relations. 0 disables logging, 1 logs only successful attempts, and 2 logs all attempts.
    #[arg(short, long, default_value_t = 1, value_parser = validate_verbose)]
    verbose: u8,

    /// Specifies the maximum number of URL relations to discover.
    #[arg(long, default_value_t = 1000)]
    max_urls: usize,

    /// A string that new URLs must contain to be considered. Default is an empty string, meaning all URLs are considered."
    #[arg(long, default_value_t = String::from(""))]
    match_str: String,

    /// A comma-separated list of strings. URLs containing any of these strings will be ignored. Default is an empty string, meaning no URLs are ignored.
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

    let ignore_str: Vec<&str> = args.ignore.split(',').map(|s| s.trim()).collect();

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
