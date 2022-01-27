use crate::tui::util::{SinSignal, StatefulList, TabsState};
use std::process::Command;
use crate::memo;
use std::fs;
use std::path;
use std::error::Error;
use winrt;

winrt::import!(
    dependencies
        os
    types
        windows::system::Launcher
        windows::application_model::data_transfer::*
);

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct Signals {
    pub sin1: Signal<SinSignal>,
    pub sin2: Signal<SinSignal>,
    pub window: [f64; 2],
}

impl Signals {
    fn on_tick(&mut self) {
        self.sin1.on_tick();
        self.sin2.on_tick();
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}

pub fn read_dir(path: &str) -> Result<Vec<path::PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<path::PathBuf> = Vec::new();
    for item in dir.into_iter() {
        files.push(item?.path());
    }
    Ok(files)
}

fn launch_file(path: &str) -> winrt::Result<()> {
    //assert!(env::set_current_dir(&Path::new("C:/Users/user/Documents/memo")).is_ok());
    let path = path.replace("/", "\\").to_string();
    println!("{}", path);
    Command::new("goneovim.exe")
        .arg(path)
        .spawn()
        .expect("failed to open memo");
    panic!("not implement");
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub show_chart: bool,
    pub progress: f64,
    pub folders: Vec<StatefulList<memo::Memo>>,
    pub enhanced_graphics: bool,
    pub folders_index: usize,
    pub path_copied: String,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, lst_memo: &Vec<memo::Memo>, enhanced_graphics: bool) -> App<'a> {

        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            show_chart: false,
            progress: 0.0,
            folders: vec![StatefulList::with_items(lst_memo.clone())] ,
            enhanced_graphics,
            folders_index: 0,
            path_copied: "".to_string(),
        }
    }

    pub fn on_up(&mut self) {
        self.folders[self.folders_index].previous();
    }

    pub fn on_down(&mut self) {
        self.folders[self.folders_index].next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_enter_dir(&mut self) {
        match self.folders[self.folders_index].state.selected() {
            Some(x) => {
                let path_target = &self.folders[self.folders_index].items[x].get_path();
                let path_target = path::Path::new(path_target);
                let path_target = path::PathBuf::from(path_target);
                launch_file(path_target.to_str().unwrap()).unwrap();
            },
            _ => {}
        }
    }

    pub fn on_focus_left_pain(&mut self) {
        self.folders_index = 0;
    }

    pub fn on_focus_right_pain(&mut self) {
        self.folders_index = 1;
    }

    pub fn search_string_in_this_path(&mut self, search: &str) {
        let lst_new: Vec<memo::Memo> = 
            self.folders[self.folders_index].items
                .clone()
                .into_iter()
                .filter(|item| 
                    item.get_path()
                    .to_lowercase()
                    .contains(&search.to_lowercase())
                ).collect();

        if let false = lst_new.is_empty() {
            self.folders[self.folders_index] = StatefulList::with_items(lst_new);
        };
        
    }

    pub fn on_key(&mut self, c: char, _: (u16, u16)) {
        match c {
            'e' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            'j' => { self.on_down(); }
            'k' => { self.on_up(); }
            'c' => { self.on_enter_dir(); }
            'l' => { self.on_focus_right_pain(); }
            'h' => { self.on_focus_left_pain(); }
            _ => {}
        }
    }

    pub fn add_task(&mut self) {

    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}
