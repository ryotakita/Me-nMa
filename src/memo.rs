use std::fmt;
use std::fs;
use std::path::{PathBuf};
use std::error::Error;

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
        write!(f, "{:<50} | tags={}", self.path, tags)
    }
}

pub fn read_dir(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    for item in dir.into_iter() {
        files.push(item?.path());
    }
    Ok(files)
}

pub fn create_memo_list() -> Vec<Memo> {
    // TODO:ファイル読み込み
    let directory = read_dir("E:/memo").unwrap();

    let files = directory.into_iter().filter(|file| file.is_file() );
    let files_md = files.filter(|file| "md" == file.extension().unwrap().to_str().unwrap() );

    files_md.filter_map(|file| create_memo_from_file(&file)).collect()
}

pub fn create_memo_from_file(file: &PathBuf) -> Option<Memo> {
    let text = match fs::read_to_string(&file) {
        Ok(text) => text,
        Err(_) =>  {
            let s = fs::read(&file).unwrap();
            let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
            res.into_owned()
        }
    };

    let lines = text.lines();

    lines.into_iter().find_map(|line| {
        match get_tags_by_line(line.to_string()) {
            Some(tags) => {
                Some(Memo::new(
                    file.to_str().unwrap().replace("\\", "/").to_string(),
                    tags,
                ))
            },
            None => None,
        }
    })

}

/// 渡されたpathに存在するmdファイルをメモとして返します。
fn get_tags_by_line(mut line: String) -> Option<Vec<String>> {
    match line.contains("tags") {
        true => {
            line.retain(|x| x != ' ');

            let mut tags: Vec<&str> = line.split('#').collect();
            tags.retain(|x| !x.contains("tags:"));
            Some(tags.iter().map(|x| x.to_string()).collect())
        }
        false => {
            None
        }
    }
}

pub fn is_include_these_tags(tags: &Vec<String>, tags_memo: &Vec<String>) -> bool {
    let mut tags_dummy = tags.clone();
    tags_dummy.retain(|tag| tags_memo.iter().all(|tag_memo| !tag.contains(tag_memo)));

    tags_dummy.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_include_these_tags_test() {
        assert_eq!(
            is_include_these_tags(
                &vec!["foo".to_string(), "bar".to_string()],
                &vec!["foo".to_string(), "bar".to_string()]
            ),
            true
        );
        assert_eq!(
            is_include_these_tags(
                &vec!["foo".to_string(), "bar".to_string()],
                &vec!["foo".to_string()]
            ),
            false
        );
        assert_eq!(
            is_include_these_tags(
                &vec!["foo".to_string()],
                &vec!["foo".to_string(), "bar".to_string()]
            ),
            true
        );
    }
}