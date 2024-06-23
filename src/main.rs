use reqwest;
use scrapper::{analise_page, extract_strings_from_html};

#[tokio::main]
async fn main() {
    analise_page("https://loyola.g12.br");
}
