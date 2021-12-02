use crate::tui::util::{RandomSignal, SinSignal, StatefulList, TabsState};
use std::fs::File;
use std::process::Command;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use once_cell::sync::OnceCell;
use main;
use crate::memo;
use std::fmt::{self, Formatter, Display};
use std::fs;
use std::path;
use std::error::Error;
use winrt;
use windows::{
    storage::StorageFile,
    system::Launcher,
};
use regex::Regex;
use fs_extra::dir::{copy, CopyOptions};

winrt::import!(
    dependencies
        os
    types
        windows::system::Launcher
        windows::application_model::data_transfer::*
);
use windows::application_model::data_transfer::{DataPackage, Clipboard};


const TASKS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

const CLIENTS: [&str; 24] = [
    "Clients1", "Clients2", "Clients3", "Clients4", "Clients5", "Clients6", "Clients7", "Clients8", "Clients9", "Clients10",
    "Clients11", "Clients12", "Clients13", "Clients14", "Clients15", "Clients16", "Clients17", "Clients18", "Clients19",
    "Clients20", "Clients21", "Clients22", "Clients23", "Clients24",
];

const DATES: [&str; 24] = [
    "21-07-1", "21-07-2", "21-07-3", "21-07-4", "21-07-5", "21-07-6", "21-07-7", "21-07-8", "21-07-9", "21-07-10",
    "21-07-11", "21-07-12", "21-07-13", "21-07-14", "21-07-15", "21-07-16", "21-07-17", "21-07-18", "21-07-19",
    "21-07-20", "21-07-21", "21-07-22", "21-07-23", "21-07-24",
];

const LOGS: [(&str, &str); 26] = [
    ("Event1", "INFO"),
    ("Event2", "INFO"),
    ("Event3", "CRITICAL"),
    ("Event4", "ERROR"),
    ("Event5", "INFO"),
    ("Event6", "INFO"),
    ("Event7", "WARNING"),
    ("Event8", "INFO"),
    ("Event9", "INFO"),
    ("Event10", "INFO"),
    ("Event11", "CRITICAL"),
    ("Event12", "INFO"),
    ("Event13", "INFO"),
    ("Event14", "INFO"),
    ("Event15", "INFO"),
    ("Event16", "INFO"),
    ("Event17", "ERROR"),
    ("Event18", "ERROR"),
    ("Event19", "INFO"),
    ("Event20", "INFO"),
    ("Event21", "WARNING"),
    ("Event22", "INFO"),
    ("Event23", "INFO"),
    ("Event24", "WARNING"),
    ("Event25", "INFO"),
    ("Event26", "INFO"),
];

const EVENTS: [(&str, u64); 24] = [
    ("B1", 9),
    ("B2", 12),
    ("B3", 5),
    ("B4", 8),
    ("B5", 2),
    ("B6", 4),
    ("B7", 5),
    ("B8", 9),
    ("B9", 14),
    ("B10", 15),
    ("B11", 1),
    ("B12", 0),
    ("B13", 4),
    ("B14", 6),
    ("B15", 4),
    ("B16", 6),
    ("B17", 4),
    ("B18", 7),
    ("B19", 13),
    ("B20", 8),
    ("B21", 11),
    ("B22", 9),
    ("B23", 3),
    ("B24", 5),
];

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
    Command::new("Code.exe")
        .arg(path)
        .spawn()
        .expect("failed to open memo");
    panic!("not implement");

    Ok(())
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub show_chart: bool,
    pub progress: f64,
    pub sparkline: Signal<RandomSignal>,
    pub folders: Vec<StatefulList<memo::Memo>>,
    pub logs: StatefulList<(&'a str, &'a str)>,
    pub signals: Signals,
    pub barchart: Vec<(&'a str, u64)>,
    pub servers: Vec<Server<'a>>,
    pub enhanced_graphics: bool,
    pub folders_index: usize,
    pub path_copied: String,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, lst_memo: &Vec<memo::Memo>, enhanced_graphics: bool) -> App<'a> {
        let mut rand_signal = RandomSignal::new(0, 100);
        let sparkline_points = rand_signal.by_ref().take(300).collect();
        let mut sin_signal = SinSignal::new(0.2, 3.0, 18.0);
        let sin1_points = sin_signal.by_ref().take(100).collect();
        let mut sin_signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let sin2_points = sin_signal2.by_ref().take(200).collect();

        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            show_chart: false,
            progress: 0.0,
            sparkline: Signal {
                source: rand_signal,
                points: sparkline_points,
                tick_rate: 1,
            },
            folders: vec![StatefulList::with_items(lst_memo.clone())] ,
            folders_index: 0,
            path_copied: "".to_string(),
            logs: StatefulList::with_items(LOGS.to_vec()),
            signals: Signals {
                sin1: Signal {
                    source: sin_signal,
                    points: sin1_points,
                    tick_rate: 5,
                },
                sin2: Signal {
                    source: sin_signal2,
                    points: sin2_points,
                    tick_rate: 10,
                },
                window: [0.0, 20.0],
            },
            barchart: EVENTS.to_vec(),
            servers: vec![
                Server {
                    name: "NorthAmerica-1",
                    location: "New York City",
                    coords: (40.71, -74.00),
                    status: "Up",
                },
                Server {
                    name: "Europe-1",
                    location: "Paris",
                    coords: (48.85, 2.35),
                    status: "Failure",
                },
                Server {
                    name: "SouthAmerica-1",
                    location: "São Paulo",
                    coords: (-23.54, -46.62),
                    status: "Up",
                },
                Server {
                    name: "Asia-1",
                    location: "Singapore",
                    coords: (1.35, 103.86),
                    status: "Up",
                },
            ],
            enhanced_graphics,
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
                launch_file(path_target.to_str().unwrap());
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
        // TODO:filterがなぜか使えない...
        let mut lst_new = Vec::new();
        for i in self.folders[self.folders_index].items.iter() {
            match i.get_path().to_lowercase().contains(&search.to_lowercase()) {
                true => {lst_new.push(i.clone());},
                false => {}
            }
        }

        match lst_new.is_empty() {
            true => {},
            false => {
                self.folders[self.folders_index] = StatefulList::with_items(lst_new);
            }
        }
        
    }

    pub fn on_key(&mut self, c: char, pos: (u16, u16)) {
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

        self.sparkline.on_tick();
        self.signals.on_tick();

        let log = self.logs.items.pop().unwrap();
        self.logs.items.insert(0, log);

        let event = self.barchart.pop().unwrap();
        self.barchart.insert(0, event);
    }
}
