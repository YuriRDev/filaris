use regex::Regex;

/// Some files are not important for us, such as images, stylesheet, etc...
/// All the following filetypes will be ignored when searching for a URL.
/// When this constant changes, don't forget to also change the test bellow. 
const IGNORE_FILETYPE: [&str; 7] = [".png", ".gif", ".jpeg", ".webp", ".svg", ".css", ".ico"];

#[derive(Debug)]
pub struct UrlData {
    pub point_to: Vec<usize>,
    pub url: String,
}

impl UrlData {
    pub fn new(url: String) -> UrlData {
        UrlData {
            point_to: Vec::new(),
            url,
        }
    }
}

pub fn normalize_url(url: String) -> String {
    let mut normalized = url.to_string();
    if url.starts_with("https://") {
        normalized = normalized.replacen("https://", "", 1);
    } else if url.starts_with("http://") {
        normalized = normalized.replacen("http://", "", 1);
    }

    if normalized.starts_with("www.") {
        normalized = normalized.replacen("www.", "", 1);
    }

    while normalized.contains("//") {
        normalized = normalized.replace("//", "/")
    }

    while normalized.ends_with('/') {
        normalized.pop();
    }
    normalized
}

pub fn validate_url(url: &str, parent_url: &str) -> Option<String> {
    let re = Regex::new(r"^(https?://|www\.|/)[^\s]*$").unwrap();
    if re.is_match(url) {
        for filetype in IGNORE_FILETYPE {
            if url.ends_with(filetype) {
                return None
            }
        }

        if url.contains("/#") {
            return None
        }

        if url.starts_with('/') {
            return Some(format!("{}{}", parent_url, url));
        }

        return Some(url.to_string());
    }
    None
}

#[cfg(test)]
mod validate_url {
    use crate::urldata::validate_url;

    #[test]
    fn invalid_urls() {
        assert_eq!(validate_url("https://my_ url.com", ""), None);
        assert_eq!(validate_url("htts://invalid.co", ""), None);
        assert_eq!(validate_url("obiously", ""), None);
        assert_eq!(validate_url("ftp::asd", ""), None);
    }

    #[test]
    fn valid_urls() {
        assert_eq!(
            validate_url("https://site.com", ""),
            Some("https://site.com".to_string())
        );
        assert_eq!(
            validate_url("https://site", ""),
            Some("https://site".to_string())
        );
        assert_eq!(
            validate_url("http://site.com", ""),
            Some("http://site.com".to_string())
        );
        assert_eq!(
            validate_url("www.site.com", ""),
            Some("www.site.com".to_string())
        );
        assert_eq!(
            validate_url("/valid_path", ""),
            Some("/valid_path".to_string())
        );
        assert_eq!(
            validate_url("/valid_path/again", ""),
            Some("/valid_path/again".to_string())
        );
    }

    #[test]
    fn concat_relative_path() {
        assert_eq!(
            validate_url("https://site.com", "https://site.com"),
            Some("https://site.com".to_string())
        );
        assert_eq!(
            validate_url("/path1", "https://site.com"),
            Some("https://site.com/path1".to_string())
        );
        assert_eq!(
            validate_url("/path1/path2", "www.site.com"),
            Some("www.site.com/path1/path2".to_string())
        );
    }

    #[test]
    fn ignore_file_type() {
        assert_eq!(validate_url("https://mysite.com/index.html", ""), Some("https://mysite.com/index.html".to_string()));
        assert_eq!(validate_url("https://mysite.com/image.png", ""), None);
        assert_eq!(validate_url("https://mysite.com/image.gif", ""), None);
        assert_eq!(validate_url("https://mysite.com/image.svg", ""), None);
    }
}

#[cfg(test)]
mod normalize_url {
    use super::*;

    #[test]
    fn normalize_https_and_www() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(normalize_url(format!("https://{site}")), site.to_string());
            assert_eq!(
                normalize_url(format!("https://www.{site}")),
                site.to_string()
            );
        }
    }

    #[test]
    fn normalize_http_and_www() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(normalize_url(format!("http://{site}")), site.to_string());
            assert_eq!(
                normalize_url(format!("http://www.{site}")),
                site.to_string()
            );
        }
    }

    #[test]
    fn normalize_end_slash() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(normalize_url(format!("http://{site}/")), site.to_string());
            assert_eq!(
                normalize_url(format!("{site}//////////////")),
                site.to_string()
            );
        }
    }

    #[test]
    fn normalize_multiple_slash() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(
                normalize_url(format!("http://{site}/////mypath////lastpath///")),
                format!("{site}/mypath/lastpath")
            );
            assert_eq!(
                normalize_url(format!("http://{site}/mypath/lastpath")),
                format!("{site}/mypath/lastpath")
            );
        }
    }
}
