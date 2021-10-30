use std::path::{PathBuf, Path};
use regex::Regex;
use structopt::clap::arg_enum;
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

struct Memo {
    path: PathBuf,
    tags: Vec<String>,
}

impl Memo {
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}
fn main() {
    //let list_memo = create_memo_list();
    let a = Opt::from_args();
    println!("{:?}", a);
}

/// 渡されたpathに存在するmdファイルをメモとして返します。
fn create_memo_list() -> Vec<Memo> {
    todo!()
}

fn is_include_these_tags(tags: Vec<String>, tags_memo: Vec<String>) -> bool {
    let mut tags_dummy = tags.clone();
    tags_dummy.retain(|tag| tags_memo.iter().all(|tag_memo| !tag.contains(tag_memo)));

    tags_dummy.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_include_these_tags_test() {
        assert_eq!(is_include_these_tags(vec!["foo".to_string(), "bar".to_string()], vec!["foo".to_string(), "bar".to_string()]), true);
        assert_eq!(is_include_these_tags(vec!["foo".to_string(), "bar".to_string()], vec!["foo".to_string()]), false);
        assert_eq!(is_include_these_tags(vec!["foo".to_string()], vec!["foo".to_string(), "bar".to_string()]), true);
    }
}