mod graph;
mod urldata;
use std::collections::VecDeque;
use std::fmt::format;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{option, thread};
use tokio::task;

use colored::Colorize;
use graph::Graph;
use regex::{NoExpand, Regex};
use reqwest::StatusCode;
use urldata::{normalize_url, validate_url, UrlData};

#[derive(Debug, Clone)]
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
pub struct Analiser {
    graph: Arc<Mutex<Graph>>,
    queue: Arc<Mutex<VecDeque<UrlQueue>>>,
}

#[derive(Debug, Clone)]
pub struct Options {
    max_depth: usize,
    max_urls: usize,
    match_str: String,
    verbose: VerboseLevel,
    ignore_strs: Vec<String>,
}

impl Options {
    pub fn new(
        max_depth: usize,
        max_urls: usize,
        match_str: String,
        verbose: VerboseLevel,
        ignore_strs: Vec<String>,
    ) -> Options {
        Options {
            max_depth,
            max_urls,
            match_str,
            verbose,
            ignore_strs,
        }
    }
}

#[derive(Debug)]
struct UrlQueue {
    depth: usize,
    url: String,
    parent: String,
}

impl Analiser {
    pub fn new(url: &str) -> Analiser {
        Analiser {
            graph: Arc::new(Mutex::new(Graph::new())),
            queue: Arc::new(Mutex::new(VecDeque::from([UrlQueue {
                depth: 0,
                url: url.to_string(),
                parent: "".to_string(),
            }]))),
        }
    }

    pub async fn start(&mut self, options: Options) {
        let concurrency_level = 4;

        let mut handles = vec![];

        for i in 0..concurrency_level {
            let options = Arc::new(options.clone());
            let queue = Arc::clone(&self.queue);
            let handle = task::spawn(async move {
                loop {
                    let url = {
                        let mut queue_lock = queue.lock().unwrap();
                        queue_lock.pop_front()
                    };

                    match url {
                        Some(url) => {
                            match process_url(&url, &options, i).await {
                                None => {
                                    // println!("Thread [{i}]: Invalid UR");
                                }
                                Some(content) => {
                                    log_new_url(content.len(), &url.parent, &url.url, i);
                                    let mut queue_lock = queue.lock().unwrap();
                                    for new_url in content {
                                        queue_lock.push_front(UrlQueue {
                                            depth: &url.depth + 1,
                                            parent: String::from(&url.url),
                                            url: new_url,
                                        })
                                    }
                                }
                            }
                            // println!("Thread [{i}]: Completed!");
                        }
                        None => tokio::time::sleep(Duration::from_millis(100)).await,
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap()
        }
    }
}

/// Log a new valid URL "connection", that is,
/// based on the VerboseLevel, prints the URL connection in the format:
///
/// ```[length] parent -> children```
fn log_new_url(urls_len: usize, parent: &str, url: &str, thread_id: usize) {
    let formated_number = match urls_len {
        0_usize..=9_usize => format!("  {}", urls_len),
        10_usize..=99_usize => format!(" {}", urls_len),
        _ => format!("{}", urls_len),
    };

    println!(
        "{} {} {} {} {}",
        thread_id,
        format!("[URLs: {}]", formated_number).green(),
        parent.italic().bright_black(),
        "———>".green(),
        url.underline()
    );
}

/// PRINTS:
///  * InvalidUrl log.
///  * Successfull URL log
///
async fn process_url(
    url: &UrlQueue,
    options: &Arc<Options>,
    thread_id: usize,
) -> Option<Vec<String>> {
    match get_page_content(&url.url).await {
        None => {
            // PRINT_INVALID
            // println!("Thread [{thread_id}]: Invalid website - Invalid HTTP status code");
            return None;
        }
        Some(content) => {
            let urls = extract_urls_from_content(&content, &url.url);
            let mut to_scan: Vec<String> = Vec::new();
            for scan in urls {
                if !scan.contains(&options.match_str) {
                    continue;
                }
                if options.ignore_strs.len() > 0 {
                    for ignore in options.ignore_strs.clone() {
                        if scan.contains(&ignore) {
                            continue;
                        }
                    }
                }
                to_scan.push(scan);
            }
            Some(to_scan)
        }
    }
}

/// Based on regex for now...
/// TODO: Write more in docs
fn extract_urls_from_content(content: &str, parent_url: &str) -> Vec<String> {
    // @TODO: If content is HTML parse
    // @TODO: If content is JS parse

    let re = Regex::new(r#"[\"'`](.*?)[\"'`]"#).unwrap();
    let mut substrings: Vec<String> = Vec::new();

    for cap in re.captures_iter(content) {
        if let Some(url) = validate_url(&cap[1], parent_url) {
            substrings.push(url.to_string());
        }
    }

    substrings
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
