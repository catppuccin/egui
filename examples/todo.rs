#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "todos",
        native_options,
        Box::new(|_| Box::new(App::default())),
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    todos: Vec<Todo>,
    new_todo: String,
    mode: Mode,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    All,
    Completed,
    #[default]
    Active,
}

impl Default for App {
    fn default() -> Self {
        const LOREM_IPSUM: &str = "Lorem ipsum
dolor sit amet
consectetur adipiscing
elit, sed do
eiusmod tempor
incididunt ut labore
et dolore magna
aliqua. Ut enim
ad minim veniam, quis
nostrud
exercitation ullamco
laboris nisi
ut aliquip ex
ea commodo consequat.
Duis aute irure
dolor in reprehenderit
in voluptate velit
esse cillum
dolore eu fugiat
nulla pariatur.
Excepteur sint
occaecat cupidatat
non proident, sunt
in culpa qui
officia deserunt
mollit anim id
est laborum.";
        Self {
            todos: LOREM_IPSUM
                .lines()
                .map(|text| Todo {
                    description: text.to_string(),
                    completed: false,
                })
                .collect(),
            new_todo: String::new(),
            mode: Mode::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
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

        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("Todos");
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

            if self.todos.len() > 0 {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.mode, Mode::Active, "Active");
                    ui.selectable_value(&mut self.mode, Mode::Completed, "Completed");
                    ui.selectable_value(&mut self.mode, Mode::All, "All");
                    let completed = self.todos.iter().filter(|todo| todo.completed).count();
                    if completed > 0 {
                        if ui
                            .button(format!("Clear completed ({})", completed))
                            .clicked()
                        {
                            self.todos.retain(|todo| !todo.completed);
                        }
                    }
                });
            }
        });
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Todo {
    description: String,
    completed: bool,
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
