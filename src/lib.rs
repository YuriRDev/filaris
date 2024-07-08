mod graph;
mod urldata;
use std::collections::VecDeque;

use colored::Colorize;
use graph::Graph;
use regex::Regex;
use reqwest::StatusCode;
use urldata::{normalize_url, validate_url, UrlData};

#[derive(Debug)]
pub enum VerboseLevel {
    None = 0,           // Only print the start and end of program
    SuccessAtempts = 1, // Only prints the success atempts
    AllAtempts = 2,     // Prints all the atempts of reaching a URL
}

impl VerboseLevel {
    pub fn from_u8(value: u8) -> VerboseLevel {
        match value {
            0 => VerboseLevel::None,
            1 => VerboseLevel::SuccessAtempts,
            _ => VerboseLevel::AllAtempts,
        }
    }
}

#[derive(Debug)]
pub struct Analiser<'s> {
    graph: Graph,
    queue: VecDeque<UrlQueue>,
    max_depth: usize,
    max_urls: usize,
    match_str: String,
    verbose: VerboseLevel,
    ignore_strs: Vec<&'s str>,
}

#[derive(Debug)]
struct UrlQueue {
    depth: usize,
    url: String,
    parent: String,
}

impl<'s> Analiser<'s> {
    pub fn new(
        url: &str,
        match_str: &str,
        max_depth: usize,
        max_urls: usize,
        verbose: VerboseLevel,
        ignore_strs: Vec<&'s str>,
    ) -> Analiser<'s> {
        Analiser {
            graph: Graph::new(),
            queue: VecDeque::from([UrlQueue {
                depth: 0,
                url: url.to_string(),
                parent: String::from(""),
            }]),
            match_str: match_str.to_string(),
            max_depth,
            max_urls,
            verbose,
            ignore_strs,
        }
    }

    fn add_to_queue(&mut self, url: &str, depth: usize, parent: String) {
        self.queue.push_back(UrlQueue {
            depth,
            url: url.to_string(),
            parent,
        })
    }

    /// Checks if a URL was already scanned and added on the Graph Url's list.
    fn already_scanned(&self, url: &str) -> bool {
        for scanned_urls in &self.graph.urls {
            if normalize_url(scanned_urls.url.clone()) == normalize_url(url.to_string()) {
                return true;
            }
        }
        false
    }

    /// Checks if a URL is already in queue.
    fn in_queue(&self, url: &str) -> bool {
        for queue_urls in &self.queue {
            if normalize_url(queue_urls.url.clone()) == normalize_url(url.to_string()) {
                return true;
            }
        }
        false
    }

    /// Log a new valid URL "connection", that is,
    /// based on the VerboseLevel, prints the URL connection in the format:
    ///
    /// ```[length] parent -> children```
    fn log_new_url(&self, urls_len: usize, parent: &str, url: &str) {
        match self.verbose {
            VerboseLevel::None => {}
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

    /// Log a invalid URL discovery, based on the VerboseLevel,
    /// prints the URL in the format:
    ///
    /// ```[Invalid] url```
    fn log_invalid_url(&self, url: &str) {
        if let VerboseLevel::AllAtempts = self.verbose {
            println!("{} {}", "[Invalid  ]".red(), url.italic().bright_black())
        }
    }

    /// Checks if a URL should be scanned, based on the
    /// `math_str` and `ignore_str` of the args
    fn should_scan_url(&self, url: &str) -> bool {
        if url.contains(&self.match_str) {
            for ignore in &self.ignore_strs {
                if url.contains(ignore) {
                    return false;
                }
            }
            return true;
        }

        true
    }

    fn extract_urls_from_content(&self, content: &str, parent_url: &str) -> Vec<String> {
        // @TODO: If content is HTML parse
        // @TODO: If content is JS parse

        let re = Regex::new(r#"[\"'`](.*?)[\"'`]"#).unwrap();
        let mut substrings: Vec<String> = Vec::new();

        for cap in re.captures_iter(content) {
            if let Some(url) = validate_url(&cap[1], parent_url) {
                if self.should_scan_url(&url) {
                    substrings.push(url.to_string());
                }
            }
        }

        substrings
    }

    pub async fn start(&mut self) {
        while let Some(url) = self.queue.pop_front() {
            if url.depth > self.max_depth {
                return;
            }

            match get_page_content(&url.url).await {
                None => self.log_invalid_url(&url.url),
                Some(content) => {
                    if self.graph.size() >= self.max_urls {
                        return;
                    }

                    let founded_urls = self.extract_urls_from_content(&content, &url.url);

                    self.log_new_url(founded_urls.len(), &url.parent, &url.url);
                    self.graph
                        .add(UrlData::new(url.url.to_string()), &url.parent);

                    for new_url in founded_urls {
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
            }
        }
    }
}

/// Requests for the page content, and return's it if the status code
/// is not 404 or 403. All the other contents for now are returned OK.
async fn get_page_content(url: &str) -> Option<String> {
    let req_result = reqwest::get(url).await;
    match req_result {
        Err(_e) => None,
        Ok(content) => {
            if content.status() == StatusCode::NOT_FOUND
                || content.status() == StatusCode::FORBIDDEN
            {
                return None;
            }
            match content.text().await {
                Err(_e) => {
                    println!("[ERROR] Failed to read html content");
                    None
                }
                Ok(value) => Some(value),
            }
        }
    }
}
