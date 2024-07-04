use clap::Parser;
use reqwest;
use scrapper::{analise_page, extract_strings_from_html, Analiser};
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
    depth: u8,

    /// Prints every new URL relation when it founds one.
    #[arg(short, long, default_value_t = true)]
    verbose: bool,

    /// Number of the max URLs relation to be found.
    #[arg(long, default_value_t = 100)]
    max_urls: u8,

    /// Every new URL Discover needs to match this URL. Here, it could be
    /// passed a domain as a plain text
    #[arg(long, default_value_t = String::from(""))]
    match_url: String,
}

fn validate_initial_url(url: &str) -> Result<String, String> {
    Url::parse(url).map(|_| url.to_string()).map_err(|_| {
        format!(
            "'{}' is not a valid URL. Make sure that it starts with \"http://\" or \"http://\"",
            url
        )
    })
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut analiser = Analiser::new(&args.url, 2, &args.match_url);
    analiser.start().await;
}
