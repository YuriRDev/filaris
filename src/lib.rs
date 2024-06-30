mod graph;
mod urldata;
use std::collections::{vec_deque, VecDeque};

use graph::Graph;
use regex::Regex;
use reqwest::Url;
use urldata::{normalize_url, validate_url, UrlData};

#[derive(Debug)]
pub struct Analiser {
    graph: Graph,
    queue: VecDeque<UrlQueue>,
    max_depth: usize,
}

#[derive(Debug)]
struct UrlQueue {
    depth: usize,
    url: String,
    parent: String
}

impl UrlQueue {
    pub fn new(url: &str, depth: usize, parent: String) -> UrlQueue {
        UrlQueue {
            url: url.to_string(),
            depth,
            parent
        }
    }
}

impl Analiser {
    pub fn new(url: &str, max_depth: usize) -> Analiser {
        Analiser {
            graph: Graph::new(),
            queue: VecDeque::from([UrlQueue {
                depth: 1,
                url: url.to_string(),
                parent: String::from("")
            }]),
            max_depth,
        }
    }

    fn add_to_queue(&mut self, url: &str, depth: usize, parent: String) {
        self.queue.push_back(UrlQueue {
            depth,
            url: url.to_string(),
            parent
        })
    }

    fn already_exists(&self, url: &str) -> bool {
        for scanned_urls in &self.graph.urls {
            if normalize_url(scanned_urls.url.clone()) == normalize_url(url.to_string()) {
                return true
            }
        }
        return false
    }

    pub async fn start(&mut self) {
        while !self.queue.is_empty() {
            if let Some(url) = self.queue.pop_front() {
                let parent = url.url.to_string();
                match analise_page(&url.url).await {
                    Some(href_url) => {
                        println!("[{}] {} ----> {}", href_url.len(), &url.parent, &url.url);
                        self.graph.add(UrlData::new(url.url.to_string()), &url.parent);
                        for new_url in href_url {
                            if !&self.already_exists(&new_url) {
                                self.add_to_queue(&new_url, url.depth + 1, url.url.to_string())
                            }
                        }
                    }
                    None => {
                        println!("[Validator] {} is invalid.", &url.url);
                    }
                }
            }
        }
    }
}

pub async fn analise_page(url: &str) -> Option<Vec<String>> {
    let req_result = reqwest::get(url).await;
    match req_result {
        Err(e) => {
            return None;
        }
        Ok(content) => match content.text().await {
            Err(e) => {
                println!("[ERROR] Failed to read html content");
                return None;
            }
            Ok(html) => return Some(extract_strings_from_html(&html, &url, "cpavalanche")),
        },
    }
}

pub fn extract_strings_from_html(text: &str, parent_url: &str, domain: &str) -> Vec<String> {
    let re = Regex::new(r#"[\"'`](.*?)[\"'`]"#).unwrap();
    let mut substrings: Vec<String> = Vec::new();

    for cap in re.captures_iter(text) {
        if let Some(url) = validate_url(&cap[1], parent_url) {
            if is_same_domain(&url, domain) {
                substrings.push(url.to_string());
            }
        }
    }

    substrings
}

fn is_same_domain(url: &str, domain: &str) -> bool {
    url.contains(domain)
}
