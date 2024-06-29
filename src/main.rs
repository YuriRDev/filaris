use reqwest;
use scrapper::{analise_page, extract_strings_from_html, Analiser};

#[tokio::main]
async fn main() {
    let analiser = Analiser::new("https://cpavalanche.net", 2);
    analiser.start().await;
}
