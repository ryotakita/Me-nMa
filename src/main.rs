use anyhow::{bail, Result};
use chrono::{Utc};
use std::env;
use encoding_rs;
use std::error::Error;
use std::io::BufReader;
use std::fs;
use serde::{Serialize, Deserialize};
use std::io::{Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::{clap, StructOpt};
winrt::import!(
    dependencies
        os
    types
        windows::system::Launcher
);

mod memo;
mod tui;
mod gui;


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
    #[structopt(name = "todo", about = "open todo.txt")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Todo {},
    #[structopt(name = "gui", about = "launch gui mode")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    GUI {},
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    println!("{:?}", args);

    // 設定ファイル読み込み
    let mut dir_exe = env::current_exe().unwrap();
    dir_exe.pop();
    env::set_current_dir(dir_exe);

    let file_setting = fs::File::open("setting.json").expect(&format!("setting.json isn't exist. Please make setting.json at {};", env::current_dir().unwrap().to_str().unwrap()));
    let reader_setting = BufReader::new(file_setting);
    let setting: memo::Setting = serde_json::from_reader(reader_setting).expect("can't read jsonfile correctly. Please ensure json format");

    let lst_memo = memo::create_memo_list(&setting.get_memo_path());

    match args.sub {
        Sub::List { tags } => {
            match tags {
                Some(tags) => loop {
                    let lst_memo_include_thesetags: Vec<memo::Memo> =
                        if tags.iter().any(|x| x.contains("all")) {
                            lst_memo.clone()
                        } else {
                            lst_memo
                                .iter()
                                .filter(|memo| memo::is_include_these_tags(&tags, memo.get_tags()))
                                .cloned()
                                .collect()
                        };
                    tui::launch_tui(&lst_memo_include_thesetags).unwrap();
                },
                None => {
                    bail!("tag value is incorrect. please input valid value.")
                }
            }
        }
        Sub::Add { title, tags } => {
            let path = Path::new("E:/memo/");
            let filename = Utc::now().format("%y%m%d_").to_string() + &title + ".md";

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
                Err(why) => panic!("Error:{}", why),
                Ok(_) => println!("finished"),
            }

            launch_file(&(path.to_str().unwrap().to_string() + &filename)).unwrap();
            Ok(())
        }
        Sub::SetPath { path: _ } => {
            bail!("this function is not implement;")
        }
        Sub::Todo {} => {
            launch_file("E:/memo/todo.md").unwrap();
            Ok(())
        }
        Sub::GUI {} => {
            let app = gui::TemplateApp::default();
            let native_options = eframe::NativeOptions::default();
            eframe::run_native(Box::new(app), native_options); 
        }
    }
}

fn launch_file(path: &str) -> winrt::Result<()> {
    //assert!(env::set_current_dir(&Path::new("C:/Users/user/Documents/memo")).is_ok());
    let path = path.replace("/", "\\").to_string();
    println!("{}", path);
    Command::new("nvim-qt.exe")
        .arg(path)
        .spawn()
        .expect("failed to open memo");

    Ok(())
}
