use itertools::Itertools;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use structopt::{clap, StructOpt};
use windows::{storage::StorageFile, system::Launcher};
winrt::import!(
    dependencies
        os
    types
        windows::system::Launcher
);

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
        #[structopt(short = "T", long = "title")]
        title: String,
        #[structopt(short = "t", long = "tags")]
        tags: Option<Vec<String>>,
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
    path: String,
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
        write!(f, "path={} | tags={}", self.path, tags)
    }
}
fn main() {
    let args = Opt::from_args();
    println!("{:?}", args);

    let lst_memo = create_memo_list();

    match args.sub {
        Sub::List { tags } => {
            match tags {
                Some(tags) => loop {
                    let lst_memo_include_thesetags: Vec<Memo> = lst_memo
                        .iter()
                        .filter(|memo| is_include_these_tags(&tags, memo.get_tags()))
                        .cloned()
                        .collect();
                    for (i, memo) in lst_memo_include_thesetags.iter().enumerate() {
                        println!("[{}]{}", i, memo);
                    }
                    println!("input open document number");
                    let mut word = String::new();
                    std::io::stdin().read_line(&mut word).ok();
                    let answer = word.trim().to_string();
                    let answer: usize = answer.parse().expect("input is not number.");
                    match answer < lst_memo_include_thesetags.len() {
                        true => {
                            launch_file(&lst_memo_include_thesetags[answer].path).unwrap();
                            std::process::exit(0);
                        }
                        false => {
                            println!("input number is incorrect.");
                        }
                    }
                },
                None => {
                    // TODO:エラーハンドリング
                    panic!("tag value is incorrect. please input valid value.")
                }
            }
        }
        Sub::Add { title, tags } => {
            let path = Path::new("C:/Users/user/Documents/memo/");
            let filename = title + ".md";

            // 複数回実行した場合上書きされる
            let mut file = match fs::File::create(path.to_str().unwrap().to_string() + &filename) {
                Err(why) => panic!("Couldn't create {}", why),
                Ok(file) => file,
            };

            let mut tags_out: String = String::new();
            match tags {
                Some(tags) => {
                    for tag in tags {
                        tags_out += &(format!("#{} ", tag));
                    }
                }
                None => {}
            };

            let contents = format!(" <!---\n tags: {}\n --->\n", tags_out);
            match file.write_all(contents.as_bytes()) {
                Err(why) => panic!("Error"),
                Ok(_) => println!("finished"),
            }

            launch_file(&(path.to_str().unwrap().to_string() + &filename));
        }
        Sub::SetPath { path } => {}
    }
}

/// 渡されたpathに存在するmdファイルをメモとして返します。
fn create_memo_list() -> Vec<Memo> {
    let mut lst_memo: Vec<Memo> = Vec::new();

    // TODO:ファイル読み込み
    let path = Path::new("C:/Users/user/Documents/memo");

    let mut files: Vec<PathBuf> = Vec::new();
    for files in read_dir("C:/Users/user/Documents/memo") {
        for file in files {
            match file.is_file() {
                true => {
                    let extension = file.extension().unwrap();
                    match extension == OsStr::new("md") || extension == OsStr::new("txt") {
                        true => {
                            for line in BufReader::new(fs::File::open(&file).unwrap()).lines() {
                                let mut line = line.unwrap();
                                if !line.contains("tags") {
                                    continue;
                                }
                                line.retain(|x| x != ' ');

                                let mut tags: Vec<&str> = line.split('#').collect();
                                tags.retain(|x| !x.contains("tags:"));
                                let tags = tags.iter().map(|x| x.to_string()).collect();

                                lst_memo.push(Memo {
                                    path: file.to_str().unwrap().replace("\\", "/").to_string(),
                                    tags: tags,
                                });
                            }
                        }
                        false => {}
                    }
                }
                // ignore this content if item isnt file
                false => {}
            }
        }
    }

    lst_memo
}

pub fn read_dir(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    for item in dir.into_iter() {
        files.push(item?.path());
    }
    Ok(files)
}

fn is_include_these_tags(tags: &Vec<String>, tags_memo: &Vec<String>) -> bool {
    let mut tags_dummy = tags.clone();
    tags_dummy.retain(|tag| tags_memo.iter().all(|tag_memo| !tag.contains(tag_memo)));

    tags_dummy.is_empty()
}

fn launch_file(path: &str) -> winrt::Result<()> {
    // ファイルパスから `StorageFile` オブジェクトを取得
    let file = StorageFile::get_file_from_path_async(path)
        .unwrap()
        .get()
        .unwrap();

    // 既定のプログラムを使用して `file` を開く
    Launcher::launch_file_async(file).unwrap().get().unwrap();
    Ok(())
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
