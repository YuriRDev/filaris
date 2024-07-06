mod graph;
mod urldata;
use std::{
    collections::{vec_deque, VecDeque},
    fmt::format,
};

use colored::Colorize;
use graph::Graph;
use regex::Regex;
use reqwest::{StatusCode, Url};
use urldata::{normalize_url, validate_url, UrlData};

#[derive(Debug)]
enum VerboseLevel {
    None = 0,           // Only print the start and end of program
    SuccessAtempts = 1, // Only prints the success atempts
    AllAtempts = 2,     // Prints all the atempts of reaching a URL
}

#[derive(Debug)]
pub struct Analiser {
    graph: Graph,
    queue: VecDeque<UrlQueue>,
    max_depth: usize,
    max_urls: usize,
    url_math: String,
    verbose: VerboseLevel,
}

#[derive(Debug)]
struct UrlQueue {
    depth: usize,
    url: String,
    parent: String,
}

impl UrlQueue {
    pub fn new(url: &str, depth: usize, parent: String) -> UrlQueue {
        UrlQueue {
            url: url.to_string(),
            depth,
            parent,
        }
    }
}

impl Analiser {
    pub fn new(url: &str, url_match: &str, max_depth: usize, max_urls: usize) -> Analiser {
        Analiser {
            graph: Graph::new(),
            queue: VecDeque::from([UrlQueue {
                depth: 0,
                url: url.to_string(),
                parent: String::from(""),
            }]),
            url_math: url_match.to_string(),
            max_depth,
            max_urls,
            verbose: VerboseLevel::AllAtempts,
        }
    }

    fn add_to_queue(&mut self, url: &str, depth: usize, parent: String) {
        self.queue.push_back(UrlQueue {
            depth,
            url: url.to_string(),
            parent,
        })
    }

    fn already_scanned(&self, url: &str) -> bool {
        for scanned_urls in &self.graph.urls {
            if normalize_url(scanned_urls.url.clone()) == normalize_url(url.to_string()) {
                return true;
            }
        }
        return false;
    }

    fn in_queue(&self, url: &str) -> bool {
        for queue_urls in &self.queue {
            if normalize_url(queue_urls.url.clone()) == normalize_url(url.to_string()) {
                return true;
            }
        }
        false
    }

    fn log_new_url(&self, urls_len: usize, parent: &str, url: &str) {
        match self.verbose {
            VerboseLevel::None => {
                return;
            }
            _ => {
                let formated_number = match urls_len {
                    0_usize..=9_usize => format!("  {}", urls_len),
                    10_usize..=99_usize => format!(" {}", urls_len),
                    _ => format!("{}", urls_len),
                };

                println!(
                    "{} {} {} {}",
                    format!("[URLs: {}]", formated_number).green(),
                    parent.italic().bright_black(),
                    "———>".green(),
                    url.underline()
                );
            }
        }
    }

    fn log_invalid_url(&self, url: &str) {
        match self.verbose {
            VerboseLevel::AllAtempts => {
                println!("{} {}", "[Invalid  ]".red(), url.italic().bright_black())
            }
            _ => {}
        }
    }

    pub async fn start(&mut self) {
        while let Some(url) = self.queue.pop_front() {
            if url.depth > self.max_depth {
                return;
            }

            match analise_page(&url.url, &self.url_math).await {
                Some(href_urls) => {
                    if self.graph.size() >= self.max_urls {
                        return;
                    }

                    self.log_new_url(href_urls.len(), &url.parent, &url.url);
                    self.graph
                        .add(UrlData::new(url.url.to_string()), &url.parent);

                    for new_url in href_urls {
                        if self.graph.size() >= self.max_urls {
                            return;
                        }

                        if self.already_scanned(&new_url) {
                            self.graph.add(UrlData::new(new_url.to_string()), &url.url);
                        } else if !self.in_queue(&new_url) {
                            self.add_to_queue(&new_url, url.depth + 1, url.url.to_string());
                        }
                    }
                }
                None => {
                    self.log_invalid_url(&url.url);
                }
            }
        }
    }
}

async fn analise_page(url: &str, url_match: &str) -> Option<Vec<String>> {
    let req_result = reqwest::get(url).await;
    match req_result {
        Err(e) => {
            return None;
        }
        Ok(content) => {
            if content.status() == StatusCode::NOT_FOUND {
                return None;
            }
            match content.text().await {
                Err(e) => {
                    println!("[ERROR] Failed to read html content");
                    return None;
                }
                Ok(html) => return Some(extract_strings_from_html(&html, &url, &url_match)),
            }
        }
    }
}

fn extract_strings_from_html(text: &str, parent_url: &str, domain: &str) -> Vec<String> {
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
