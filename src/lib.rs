mod graph;
mod urldata;
use graph::Graph;
use regex::Regex;
use reqwest::Url;
use urldata::normalize_url;

#[derive(Debug)]
struct UrlData {
    point_to: Vec<usize>,
    url: String,
}

#[derive(Debug)]
pub struct Analiser {
    graph: Graph,
    queue: Vec<UrlQueue>,
    max_depth: usize,
}

#[derive(Debug)]
struct UrlQueue {
    depth: usize,
    url: String
}

impl Analiser {
    pub fn new(url: &str, max_depth: usize) -> Analiser {
        Analiser {
            graph: Graph::new(),
            queue: vec![UrlQueue{
                depth: 1,
                url: url.to_string()
            }],
            max_depth,
        }
    }

    fn add_to_queue(&mut self, url: String, depth: usize) {
        self.queue.push(UrlQueue {
            depth,
            url 
        })
    }

    pub async fn start(&mut self) {
        let mut depth = 1;
        for url in &self.queue {
            let parent = url.url.to_string();
            match analise_page(&url.url).await {
                Some(href_url) => {
                    for new_url in href_url {
                        println!("{}", new_url);
                    }
                },
                None => {
                    println!("Site invalido")
                } 
            } 
        }
    }
}

pub async fn analise_page(url: &str) -> Option<Vec<String>> {
    println!("called! {}", url);
    let reqResult = reqwest::get(url).await;
    match reqResult {
        Err(e) => {
            panic!("[ERROR] Could not find website, check if the URL is correct");
            return None;
        }
        Ok(content) => match content.text().await {
            Err(e) => {
                panic!("[ERROR] Failed to read html content");
                return None;
            }
            Ok(html) => return Some(extract_strings_from_html(&html, false, "cpavalanche.net")),
        },
    }
}

pub fn extract_strings_from_html(text: &str, same_domain: bool, domain: &str) -> Vec<String> {
    let re = Regex::new(r#"[\"'`](.*?)[\"'`]"#).unwrap();
    let mut substrings: Vec<String> = Vec::new();

    for cap in re.captures_iter(text) {
        if is_valid_url(&cap[1]) {
            if same_domain {
                if is_same_domain(&cap[1], domain) {
                    substrings.push(cap[1].to_string());
                }
            } else {
                substrings.push(cap[1].to_string());
            }
        }
    }

    substrings
}

fn is_valid_url(url: &str) -> bool {
    let re = Regex::new(r"^(https?://|/)[^\s]*$").unwrap();
    re.is_match(url)
}

fn is_same_domain(url: &str, domain: &str) -> bool {
    url.contains(domain)
}
