use regex::Regex;
mod graph;

#[derive(Debug)]
struct UrlData {
    point_to: Vec<usize>,
    url: String,
}





pub async fn analise_page(url: &str) {
    let reqResult = reqwest::get(url).await;
    match reqResult {
        Err(e) => {
            panic!("[ERROR] Could not find website, check if the URL is correct");
        }
        Ok(content) => match content.text().await {
            Err(e) => {
                panic!("[ERROR] Failed to read html content");
            }
            Ok(html) => {
                extract_strings_from_html(&html, true, "loyola.g12.br");
            }
        },
    }
}

pub fn extract_strings_from_html(text: &str, same_domain: bool, domain: &str) {
    let re = Regex::new(r#"[\"'`](.*?)[\"'`]"#).unwrap();
    let mut substrings: Vec<String> = Vec::new();

    for cap in re.captures_iter(text) {
        if is_valid_url(&cap[1]) {
            // TO-DO: Abstract this later
            if cap[1].ends_with(".png")
                || cap[1].ends_with(".gif")
                || cap[1].ends_with(".jpeg")
                || cap[1].ends_with(".webp")
                || cap[1].ends_with(".svg")
            {
                continue;
            }

            if same_domain {
                if is_same_domain(&cap[1], domain) {
                    println!("{}", &cap[1]);
                }
            } else {
                println!("{}", &cap[1]);
            }
        }
    }
}

fn is_valid_url(url: &str) -> bool {
    let re = Regex::new(r"^(https?://|/)[^\s]*$").unwrap();
    re.is_match(url)
}

fn is_same_domain(url: &str, domain: &str) -> bool {
    url.contains(domain)
}
