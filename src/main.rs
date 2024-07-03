use reqwest;
use scrapper::{analise_page, extract_strings_from_html, Analiser};

#[tokio::main]
async fn main() {
    let mut analiser = Analiser::new("https://en.wikipedia.org/wiki/United_States", 2);
    analiser.start().await;
}
