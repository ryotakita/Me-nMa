use eframe::{
    egui::{self, FontDefinitions, FontFamily, FontData, ScrollArea},
    epi,
};
use std::fs::{read_to_string, read};
use crate::memo;
mod easy_mark;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
    search: String,
    lst_memo: Vec<memo::Memo>,
    path_of_show: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            search: "".to_owned(),
            lst_memo: Vec::new(),
            path_of_show: "".to_owned(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "menma"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = FontDefinitions::default();
        let fontdata = FontData{font: std::borrow::Cow::Borrowed(include_bytes!("../fonts/NotoSansJP-Regular.otf")),index:0};
        fonts.font_data.insert(
            "my_font".to_owned(),
            fontdata,
        );
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        _ctx.set_fonts(fonts);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { label, value , search, lst_memo, path_of_show} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("search tags");
                let response = ui.add(egui::TextEdit::singleline(&mut *search));
                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    *lst_memo = memo::create_memo_list();
                    let tag = vec![search.to_string()];
                    *lst_memo =
                        if tag.iter().any(|x| x.contains("all")) {
                            lst_memo.clone()
                        } else {
                            lst_memo
                                .iter()
                                .filter(|memo| memo::is_include_these_tags(&tag, memo.get_tags()))
                                .cloned()
                                .collect()
                    };
                }
            });
        });

        egui::SidePanel::left("MemoList").show(ctx, |ui| {
            ui.heading("MemoList");

            let mut selected_candidate: Vec<egui::Response>= Vec::new();
            let lst_memo_: &Vec<memo::Memo> = &lst_memo.clone();
            for memo in lst_memo {
                // TODO:ドラッグの実装
                let response = ui.add(egui::TextEdit::singleline(&mut memo.get_path().clone()));
                selected_candidate.push(response);
            }
            
            for (i, candidate) in selected_candidate.iter().enumerate() {
                if candidate.clicked() {
                    *path_of_show = lst_memo_[i].get_path().clone();
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Preview");
            // The central panel the region left after adding TopPanel's and SidePanel's
            ScrollArea::vertical().show(ui, |ui| {
                if path_of_show != "" {
                    let contents = match read_to_string(&*path_of_show) {
                        Ok(content) => content,
                        Err(_) => {
                            let s = read(&*path_of_show).unwrap();
                            let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
                            res.into_owned()
                        }
                    };
                    easy_mark::easy_mark(ui, &contents);
                }
            });
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}