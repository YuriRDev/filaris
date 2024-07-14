mod urldata;
use std::collections::VecDeque;
use std::fmt::format;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{option, thread};
use tokio::task;

use colored::Colorize;
use regex::{NoExpand, Regex};
use reqwest::StatusCode;
use urldata::{normalize_url, validate_url, UrlData};

#[derive(Debug)]
pub struct Analiser {
    /// Number of already scanned urls. It's used for the
    /// break-point at `max_urls`
    scanned: Arc<Mutex<usize>>,
    queue: Arc<Mutex<VecDeque<UrlQueue>>>,
}

#[derive(Debug, Clone)]
pub struct Options {
    max_urls: usize,
    match_str: String,
    ignore_strs: Vec<String>,
    concurrency: usize,
}

impl Options {
    pub fn new(
        max_urls: usize,
        match_str: String,
        ignore_strs: Vec<String>,
        concurrency: usize,
    ) -> Options {
        Options {
            max_urls,
            match_str,
            ignore_strs,
            concurrency,
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
            scanned: Arc::new(Mutex::new(0)),
            queue: Arc::new(Mutex::new(VecDeque::from([UrlQueue {
                depth: 0,
                url: url.to_string(),
                parent: "".to_string(),
            }]))),
        }
    }

    pub async fn start(&mut self, options: Options) {
        let mut handles = vec![];

        for i in 0..options.concurrency {
            let options = Arc::new(options.clone());
            let queue = Arc::clone(&self.queue);
            let scanned = Arc::clone(&self.scanned);
            let handle = task::spawn(async move {
                'main: loop {
                    let url = {
                        let mut queue_lock = queue.lock().unwrap();
                        queue_lock.pop_front()
                    };

                    match url {
                        Some(url) => {
                            match process_url(&url, &options).await {
                                None => {}
                                Some(content) => {
                                    // @todo: Add a new breaking-point based of depth.
                                    // Thread breaking-point.
                                    {
                                        let mut scanned_lock = scanned.lock().unwrap();
                                        *scanned_lock += 1;
                                        if *scanned_lock > options.max_urls {
                                            break 'main;
                                        }
                                    }
                                    log_new_url(content.len(), &url.parent, &url.url);
                                    let mut queue_lock = queue.lock().unwrap();
                                    for new_url in content.into_iter().rev() {
                                        queue_lock.push_back(UrlQueue {
                                            depth: &url.depth + 1,
                                            parent: String::from(&url.url),
                                            url: new_url,
                                        })
                                    }
                                }
                            }
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
/// prints the URL connection in the format:
///
/// ```[length] parent -> children```
fn log_new_url(urls_len: usize, parent: &str, url: &str) {
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

/// Receives a URL and Options, it filters all the
/// inner urls based on the Options, such as `--match-str`
/// and `--ignore`
async fn process_url(url: &UrlQueue, options: &Arc<Options>) -> Option<Vec<String>> {
    match get_page_content(&url.url).await {
        None => {
            return None;
        }
        Some(content) => {
            let urls = extract_urls_from_content(&content, &url.url);
            let mut to_scan: Vec<String> = Vec::new();
            'outer: for scan in urls {
                if !scan.contains(&options.match_str) {
                    continue;
                }
                for ignore in options.ignore_strs.clone() {
                    if scan.contains(&ignore) {
                        continue 'outer;
                    }
                }
                to_scan.push(scan);
            }
            Some(to_scan)
        }
    }
}

/// Extracts all string literals and validate if it's a URL.
/// This approach will be replaced by HTML and JS parser in the future.
fn extract_urls_from_content(content: &str, parent_url: &str) -> Vec<String> {
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
