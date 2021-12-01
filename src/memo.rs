use std::fmt;

#[derive(Debug, Clone)]
pub struct Memo {
    path: String,
    tags: Vec<String>,
}

impl Memo {
    pub fn new(path: String, tags: Vec<String>) -> Self {
        Memo {
            path: path,
            tags: tags,
        }
    }
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }
}

impl fmt::Display for Memo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // タグの配列を作る
        let tags: String = self
            .get_tags()
            .iter()
            .map(|s| s.trim().to_owned() + ", ")
            .collect();
        write!(f, "path={} | tags={}", self.path, tags)
    }
}
