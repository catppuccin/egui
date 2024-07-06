//! A simple demo todo app for the theme previews.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let app = App::default();
    if let Err(e) = eframe::run_native("todos", native_options, Box::new(|_| Ok(Box::new(app)))) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct App {
    todos: Vec<Todo>,
    new_todo: String,
    mode: Mode,
    theme: CatppuccinTheme,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CatppuccinTheme {
    Frappe,
    Latte,
    Macchiato,
    Mocha,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Todo {
    description: String,
    completed: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    All,
    Completed,
    Active,
}

impl Default for App {
    fn default() -> Self {
        let ipsum = [
            "Lorem ipsum",
            "dolor sit amet",
            "consectetur adipiscing",
            "elit, sed do",
            "eiusmod tempor",
            "incididunt ut labore",
            "et dolore magna",
            "aliqua. Ut enim",
            "ad minim veniam, quis",
            "nostrud",
            "exercitation ullamco",
            "laboris nisi",
            "ut aliquip ex",
            "ea commodo consequat.",
            "Duis aute irure",
            "dolor in reprehenderit",
            "in voluptate velit",
            "esse cillum",
            "dolore eu fugiat",
            "nulla pariatur.",
            "Excepteur sint",
            "occaecat cupidatat",
            "non proident, sunt",
            "in culpa qui",
            "officia deserunt",
            "mollit anim id",
            "est laborum.",
        ];
        Self {
            todos: ipsum
                .into_iter()
                .enumerate()
                .map(|(i, text)| Todo {
                    description: text.to_string(),
                    completed: i > 21,
                })
                .collect(),
            new_todo: String::new(),
            mode: Mode::default(),
            theme: CatppuccinTheme::Mocha,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(
            ctx,
            match self.theme {
                CatppuccinTheme::Frappe => FRAPPE,
                CatppuccinTheme::Latte => LATTE,
                CatppuccinTheme::Macchiato => MACCHIATO,
                CatppuccinTheme::Mocha => MOCHA,
            },
        );
        ctx.set_pixels_per_point(1.25);

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "open-sans".to_string(),
            egui::FontData::from_static(include_bytes!("./OpenSans-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "open-sans".to_string());
        ctx.set_fonts(fonts);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].heading("Todos");
                columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.theme))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.theme, CatppuccinTheme::Latte, "Latte");
                            ui.selectable_value(&mut self.theme, CatppuccinTheme::Frappe, "Frappe");
                            ui.selectable_value(
                                &mut self.theme,
                                CatppuccinTheme::Macchiato,
                                "Macchiato",
                            );
                            ui.selectable_value(&mut self.theme, CatppuccinTheme::Mocha, "Mocha");
                        });
                });
            });

            ui.horizontal(|ui| {
                if ui.text_edit_singleline(&mut self.new_todo).lost_focus()
                    && !self.new_todo.is_empty()
                {
                    self.todos.push(Todo::new(self.new_todo.clone()));
                    self.new_todo.clear();
                }
                if ui.button("Create").clicked() && !self.new_todo.is_empty() {
                    self.todos.push(Todo::new(self.new_todo.clone()));
                    self.new_todo.clear();
                }
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(
                    ui.available_height() - ui.text_style_height(&egui::TextStyle::Body) * 2.0,
                )
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    for todo in self.todos.iter_mut().rev() {
                        match (self.mode, todo.completed) {
                            (Mode::Active, false) | (Mode::Completed, true) | (Mode::All, _) => {
                                todo.show(ui);
                            }
                            _ => (),
                        }
                    }
                });

            if !self.todos.is_empty() {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.mode, Mode::Active, "Active");
                    ui.selectable_value(&mut self.mode, Mode::Completed, "Completed");
                    ui.selectable_value(&mut self.mode, Mode::All, "All");
                    let completed = self.todos.iter().filter(|todo| todo.completed).count();
                    if completed > 0
                        && ui
                            .button(format!("Clear completed ({})", completed))
                            .clicked()
                    {
                        self.todos.retain(|todo| !todo.completed);
                    }
                });
            }
        });
    }
}

impl Todo {
    fn new(description: String) -> Self {
        Self {
            description,
            completed: false,
        }
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        let text = if self.completed {
            egui::RichText::from(&self.description)
                .strikethrough()
                .weak()
        } else {
            egui::RichText::from(&self.description)
        };
        ui.checkbox(&mut self.completed, text);
    }
}
