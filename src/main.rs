use reqwest;
use scrapper::{analise_page, extract_strings_from_html};

#[tokio::main]
async fn main() {
    println!("{}",normalize_url("https://localhost".to_string()));
}


pub fn normalize_url(url: String) -> String {
    let mut normalized = url.to_string();
    if url.starts_with("https://") {
        normalized = normalized.replacen("https://", "", 1);
    } else if url.starts_with("http://") {
        normalized = normalized.replacen("http://", "", 1);
    }

    if normalized.starts_with("www") {
        normalized = normalized.replacen("www", "", 1);
    }

    // while (last_char == '/') REMOVE_LAST_CHAR()

    // return new str
    normalized
}
