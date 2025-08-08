use crate::*;

enum Page {
    Home,
    AllWins,
}

#[derive(PartialEq)]
pub enum IdType {
    Code,
    Nickname,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    page: Page,

    #[serde(skip)]
    id_type: IdType,

    #[serde(skip)]
    identity_input: String,

    #[serde(skip)]
    games: Vec<Game>,

    #[serde(skip)]
    slippi_path: Option<String>,

    #[serde(skip)]
    show_win_popup: bool,

    #[serde(skip)]
    total_wins: Option<i32>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            page: Page::Home,
            id_type: IdType::Nickname,
            identity_input: String::new(),
            games: Vec::new(),
            slippi_path: None,
            show_win_popup: false,
            total_wins: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.page {
                Page::Home => {
                    ui.heading("Main Menu");

                    if ui.button("Get Total Wins").clicked() {
                        self.page = Page::AllWins;
                    }

                    ui.add_space(10.0);

                    // Load via button
                    if ui.button("Load Games").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.slippi_path = Some(path.display().to_string());
                            self.games = scan_dir(&self.slippi_path.as_ref().unwrap());
                        }
                    }

                    // Load via drag-and-drop
                    if let Some(path) = ctx.input(|i| {
                        i.raw.dropped_files.iter().find_map(|f| f.path.clone())
                    }) {
                        self.slippi_path = Some(path.to_string_lossy().to_string());
                        self.games = scan_dir(&self.slippi_path.as_ref().unwrap());
                    }

                    if !self.games.is_empty() {
                        if let Some(path) = &self.slippi_path {
                            ui.label(format!("Loaded folder: {}", path));
                        }
                    }
                }
                Page::AllWins => {
                    ui.heading("Get Total Wins");

                    ui.horizontal(|ui| {
                        ui.label("ID Type:");
                        ui.radio_value(&mut self.id_type, IdType::Nickname, "Nickname");
                        ui.radio_value(&mut self.id_type, IdType::Code, "Code");
                    });

                    ui.add_space(10.0);
                    ui.label("Enter identity:");
                    ui.text_edit_singleline(&mut self.identity_input);

                    ui.add_space(10.0);
                    if ui.button("Submit").clicked() {
                        // You can call your get_total_wins function here
                        // let wins = get_total_wins(self.games.clone(), self.id_type, self.identity_input.clone());
                        // println!("Total wins: {}", wins);
                        let id = match self.id_type {
                            IdType::Nickname => Id::Nickname(self.identity_input.clone()),
                            IdType::Code => Id::Code(self.identity_input.clone()),
                        };

                        let wins: i32 = get_total_wins(&self.games, &id);
                        self.total_wins = Some(wins);
                        self.show_win_popup = true;
                    }

                    if self.show_win_popup {
                        egui::Window::new("Total Wins")
                            .collapsible(false)
                            .resizable(false)
                            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                            .show(ctx, |ui| {
                                if let Some(wins) = self.total_wins {
                                    ui.label(format!("Total Wins: {}", wins));
                                } else {
                                    ui.label("No data available.");
                                }

                                if ui.button("Close").clicked() {
                                    self.show_win_popup = false;
                                }
                            });
                    }

                    ui.add_space(20.0);
                    if ui.button("Back").clicked() {
                        self.page = Page::Home;
                    }
                }
            }
        });
    }
}
