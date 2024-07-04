use reqwest;
use scrapper::{analise_page, extract_strings_from_html, Analiser};

#[tokio::main]
async fn main() {
    let mut analiser = Analiser::new("https://github.com/YuriRDev", 2);
    analiser.start().await;
}
