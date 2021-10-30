use itertools::Itertools;
use std::fmt;
use std::path::{Path, PathBuf};
use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "MenMa")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(subcommand)]
    pub sub: Sub,
}

#[derive(Debug, StructOpt)]
pub enum Sub {
    #[structopt(name = "list", about = "view list")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    List {
        #[structopt(short = "t", long = "tags")]
        tags: Option<Vec<String>>,
    },
    #[structopt(name = "add", about = "add memo")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Add {
        #[structopt(short = "t", long = "title")]
        title: String,
    },
    #[structopt(name = "setpath", about = "set path of memo exist directory")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    SetPath {
        #[structopt(short = "p", long = "path")]
        path: PathBuf,
    },
}

#[derive(Debug, Clone)]
struct Memo {
    path: PathBuf,
    tags: Vec<String>,
}

impl Memo {
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}

impl fmt::Display for Memo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // タグの配列を作る
        let tags: String = self
            .get_tags()
            .iter()
            .map(|s| s.trim())
            .intersperse(", ")
            .collect();
        write!(
            f,
            "path:{} | tags={}",
            self.path.to_str().unwrap().to_string(),
            tags
        )
    }
}
fn main() {
    let args = Opt::from_args();
    println!("{:?}", args);

    let lst_memo = create_memo_list();

    match args.sub {
        Sub::List { tags } => {
            match tags {
                Some(tags) => {
                    let lst_memo_include_thesetags: Vec<Memo> = lst_memo
                        .iter()
                        .filter(|memo| is_include_these_tags(&tags, memo.get_tags()))
                        .cloned()
                        .collect();
                    for memo in lst_memo_include_thesetags {
                        println!("{}", memo);
                    }
                }
                None => {
                    // TODO:エラーハンドリング
                    panic!("tag value is incorrect. please input valid value.")
                }
            }
        }
        Sub::Add { title } => {}
        Sub::SetPath { path } => {}
    }
}

/// 渡されたpathに存在するmdファイルをメモとして返します。
fn create_memo_list() -> Vec<Memo> {
    let mut lst_memo: Vec<Memo> = Vec::new();
    for _ in 0..10 {
        lst_memo.push(Memo {
            path: PathBuf::new(),
            tags: vec![String::from("test"), String::from("math")],
        });
    }
    lst_memo
}

fn is_include_these_tags(tags: &Vec<String>, tags_memo: &Vec<String>) -> bool {
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
