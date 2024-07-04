use regex::Regex;
use reqwest::get;
use url::{Host, Position, Url};

/// Some files are not important for us, such as images, stylesheet, etc...
/// All the following filetypes will be ignored when searching for a URL.
/// When this constant changes, don't forget to also change the test bellow.
const IGNORE_FILETYPE: [&str; 8] = [
    ".png", ".gif", ".jpeg", ".webp", ".svg", ".css", ".ico", ".jpg",
];

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

/// Normalizes URLs returning it's `host` + `path` **always ending with `/`**
///
/// **A valid URL includes the http/https protocol in it**
///
/// This method won't including the initial `www.` prefix of the website.
pub fn normalize_url(url: String) -> String {
    let parsed_url = Url::parse(&url).unwrap();
    let host = parsed_url.host_str().unwrap().replace("www.", "");
    let path = parsed_url.path();

    format!("{host}{path}")
}

/// Check if a URL is valid, if so, returns a valid URL. 
/// 
/// E.g. joining `url`'s relative path to it's parent. If not, returns None 
/// 
/// * `url`: URL or a relative path (`/`, `../`).
/// * `parent_url`: URL that "discovered" the `url`. 
/// 
/// If `url` is a valid URL, `parent_url` will not be evaluated.
/// 
pub fn validate_url(url: &str, parent_url: &str) -> Option<String> {
    for filetype in IGNORE_FILETYPE {
        if url.ends_with(filetype) {
            return None;
        }
    }

    if let Ok(parsed_url) = Url::parse(url) {
        if parsed_url.scheme() != "https" && parsed_url.scheme() != "http" {
            return None;
        }
        return Some(url.replace("www.", ""));
    }

    let parsed_parent_url = Url::parse(parent_url);
    match parsed_parent_url {
        Ok(valid_url) => match valid_url.join(url) {
            Ok(joined_url) => return Some(joined_url.to_string().replace("www.", "")),
            _ => return None,
        },
        _ => return None,
    }

    None
}

/// Returns the base_url, AKA host, from the URL.
///
/// **A valid URL includes the http/https protocol in it**
///
/// This method won't including the initial `www.` prefix of the website.
pub fn get_base_url(url: &str) -> String {
    let parsed_url = Url::parse(url).unwrap();
    let host = parsed_url.host_str().unwrap().replace("www.", "");

    format!("{}", host)
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
        assert_eq!(validate_url("/valid_path", ""), None);
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
            validate_url("http://www.site.com", ""),
            Some("http://site.com".to_string())
        );
        assert_eq!(
            validate_url("http://site.com/path1?q=10", ""),
            Some("http://site.com/path1?q=10".to_string())
        );
        assert_eq!(
            validate_url("http://www.site.com/valid_path/again", ""),
            Some("http://site.com/valid_path/again".to_string())
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
            validate_url("/path1/path2", "http://www.site.com"),
            Some("http://site.com/path1/path2".to_string())
        );
    }

    #[test]
    fn ignore_file_type() {
        assert_eq!(
            validate_url("https://mysite.com/index.html", ""),
            Some("https://mysite.com/index.html".to_string())
        );
        assert_eq!(validate_url("https://mysite.com/image.png", ""), None);
        assert_eq!(validate_url("https://mysite.com/image.gif", ""), None);
        assert_eq!(validate_url("https://mysite.com/image.svg", ""), None);
    }
}

#[cfg(test)]
mod get_base_url {
    use super::*;

    #[test]
    fn with_http() {
        let base_site = "site.com.br";
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1/test2")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1/test2")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1/test2/test4#?41223")),
            base_site.to_string()
        );
    }

    #[test]
    fn without_http() {
        let base_site = "site.com.br";
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1/test2")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1/test2/test4#?41223")),
            base_site.to_string()
        );
    }

    #[test]
    fn query_params() {
        let base_site = "site.com.br";
        assert_eq!(
            get_base_url(&format!("https://{base_site}?test1")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1?test=2")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/?other_test=2")),
            base_site.to_string()
        );
    }

    #[test]
    fn complex_urls() {
        let base_site = "site.com.br";
        assert_eq!(
            get_base_url(&format!("https://{base_site}?test1")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/test1?test=2")),
            base_site.to_string()
        );
        assert_eq!(
            get_base_url(&format!("https://{base_site}/?other_test=2")),
            base_site.to_string()
        );
    }
}

#[cfg(test)]
mod normalize_url {
    use super::*;

    #[test]
    fn normalize_https_and_www() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(
                normalize_url(format!("https://{site}")),
                format!("{}/", site.to_string())
            );
            assert_eq!(
                normalize_url(format!("https://www.{site}")),
                format!("{}/", site.to_string())
            );
        }
    }

    #[test]
    fn normalize_query_params() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(
                normalize_url(format!("https://{site}/#div_to")),
                format!("{}/", site.to_string())
            );
            assert_eq!(
                normalize_url(format!("https://www.{site}?q=20")),
                format!("{}/", site.to_string())
            );
        }
    }

    #[test]
    fn normalize_multiple_slash() {
        // HTTP RFC 2396 (https://www.ietf.org/rfc/rfc2396.txt) defined multiple_slashes
        // to be valid and depending on the server implementation non-equivalent.
        // - site.com/path1 != site.com////path1
        // But, if the last character is not a slash_bar, if including it, will be equivalent.
        // - site.com/path1 == site.com/path1/
        let sites = ["site.com/", "othersite.com/", "moreonesite.com/"];
        for site in sites {
            assert_eq!(
                normalize_url(format!("http://{site}/////mypath////lastpath///")),
                format!("{site}/////mypath////lastpath///")
            );
            assert_eq!(
                normalize_url(format!("http://{site}/mypath/lastpath")),
                format!("{site}/mypath/lastpath")
            );
        }
    }
}
