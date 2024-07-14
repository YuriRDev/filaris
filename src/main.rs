use clap::Parser;
use scrapper::{Analiser, Options};
use url::Url;

/// A web scraper tool to explore and trace pathways within websites.
#[derive(Parser, Debug)]
#[command(version = "1.0.0")]
struct Args {
    /// The initial URL to start scanning from. Ensure it starts with 'http' or 'https'.
    #[arg(short, long, value_parser = validate_initial_url)]
    url: String,

    /// Defines how deep the BFS should go.
    // DISABLED - Waiting for priorityQueue. 
    // #[arg(short, long, default_value_t = 3)]
    // depth: usize,

    /// Specifies the maximum number of URL relations to discover.
    #[arg(long, default_value_t = 1000)]
    max_urls: usize,

    /// A string that new URLs must contain to be considered. Default is an empty string, meaning all URLs are considered."
    #[arg(long, default_value_t = String::from(""))]
    match_str: String,

    /// A comma-separated list of strings. URLs containing any of these strings will be ignored. Default is an empty string, meaning no URLs are ignored.
    #[arg(short, long, default_value_t = String::from(""))]
    ignore: String,
    
    /// Number of tasks that will be spawned. 
    #[arg(short, long, default_value_t = 5)]
    concurrency: usize,
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

    let ignore_str: Vec<String> = args
        .ignore
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let options = Options::new(
        args.depth,
        args.max_urls,
        args.match_str,
        ignore_str,
        args.concurrency
    );

    let mut analiser = Analiser::new(&args.url);
    analiser.start(options).await;
}
