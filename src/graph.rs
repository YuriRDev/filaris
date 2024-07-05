use crate::urldata::UrlData;

#[derive(Debug)]
pub struct Graph {
    pub urls: Vec<UrlData>,
}

impl Graph {
    #[must_use]
    pub fn new() -> Graph {
        Graph { urls: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.urls.len()
    }

    fn add_or_get_pos(&mut self, info: UrlData) -> usize {
        for i in 0..self.urls.len() {
            if self.urls[i].url == info.url {
                return i;
            };
        };
        self.urls.push(info);
        self.urls.len() - 1
    }

    fn get_parent_post(&self, url: &str) -> Option<usize>{
        for i in 0..self.urls.len() {
            if self.urls[i].url == url {
                return Some(i)
            }
        }
        None
    }

    pub fn add(&mut self, info: UrlData, parent_url: &str) {
        let should_point_to = self.add_or_get_pos(info);
        if let Some(i) = self.get_parent_post(parent_url) {
            self.urls[i].point_to.push(should_point_to);
        };

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_independent_urls() {
        let mut graph = Graph::new();

        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "first_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 1);
        
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "second_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 2);
        
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "third_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 3);
    }

    #[test]
    fn add_samename_urls() {
        let mut graph = Graph::new();

        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "first_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 1);
        
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "first_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 1);
        
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "second_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 2);

        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "third_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 3);
        
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "second_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 3);
        
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "first_url".to_string(),
            },
            "",
        );
        assert_eq!(graph.urls.len(), 3);
    }

    #[test]
    fn add_children_urls() {
        let mut graph = Graph::new();

        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "first_url".to_string(),
            },
            "",
        );
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "second_url".to_string(),
            },
            "first_url",
        );
        
        // Making sure that the first_url (position 0) points to the second_url (position 1)
        assert!(graph.urls[0].point_to.contains(&1));
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "third_url".to_string(),
            },
            "first_url",
        );
        // Making sure that the first_url (position 0) points to the third_url (position 2)
        assert!(graph.urls[0].point_to.contains(&2));
        graph.add(
            UrlData {
                point_to: Vec::new(),
                url: "first_url".to_string(),
            },
            "third_url",
        );
        // Making sure that the third_url (position 2) points to the first_url (position 0)
        assert!(graph.urls[2].point_to.contains(&0));
    }

}
