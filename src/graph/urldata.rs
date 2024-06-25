#[derive(Debug)]
struct UrlData {
    pub point_to: Vec<usize>,
    pub url: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_https_and_www() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(normalize_url(format!("https://{site}")), site.to_string());
            assert_eq!(normalize_url(format!("https://www.{site}")), site.to_string());
        }
    }

    #[test]
    fn normalize_http_and_www() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(normalize_url(format!("http://{site}")), site.to_string());
            assert_eq!(normalize_url(format!("http://www.{site}")), site.to_string());
        }
    }

    #[test]
    fn normalize_end_slash() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(normalize_url(format!("http://{site}/")), site.to_string());
            assert_eq!(normalize_url(format!("{site}//////////////")), site.to_string());
        }
    }

    #[test]
    fn normalize_multiple_slash() {
        let sites = ["site.com", "othersite.com", "moreonesite.com"];
        for site in sites {
            assert_eq!(normalize_url(format!("http://{site}/////mypath////lastpath///")), format!("{site}/mypath/lastpath"));
            assert_eq!(normalize_url(format!("http://{site}/mypath/lastpath")), format!("{site}/mypath/lastpath"));
        }
    }
}
