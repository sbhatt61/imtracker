use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use eframe::egui;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Project {
    name: String,
    id: usize,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    nav: u32,
    running: bool,
    start: Option<std::time::Instant>,
    proj_id: usize,
    projects: std::vec::Vec<Project>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut projects = std::vec::Vec::new();
        if let Ok(lines) = read_lines("projects.txt") {
            // Consumes the iterator, returns an (Optional) String
            let mut i = 0;
            for line in lines {
                if let Ok(project) = line {
                    projects.push(Project {
                        id: i,
                        name: project,
                    });
                    i += 1;
                }
            }
        }
        Self {
            nav: 0,
            running: false,
            start: None,
            projects: projects,
            proj_id: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Console").clicked() {
                    self.nav = 0
                }
                if ui.button("Editor").clicked() {
                    self.nav = 1
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.running {
                if ui
                    .add(egui::Button::new("Start").fill(egui::Color32::from_rgb(0, 255, 0)))
                    .clicked()
                {
                    self.running = true;
                    self.start = Some(std::time::Instant::now());
                }
            } else {
                ui.label(match self.start {
                    Some(x) => x.elapsed().as_secs().to_string(),
                    None => "Error".to_string(),
                });
                if ui
                    .add(egui::Button::new("Stop").fill(egui::Color32::from_rgb(255, 0, 0)))
                    .clicked()
                {
                    self.running = false;
                }
            }
            egui::ComboBox::from_label("Project")
                .selected_text(format!(
                    "{:?}",
                    match self.projects.get(self.proj_id) {
                        Some(x) => x.name.clone(),
                        None => "".to_string(),
                    }
                ))
                .show_ui(ui, |ui| {
                    for project in self.projects.iter() {
                        ui.selectable_value(&mut self.proj_id, project.id, project.name.clone());
                    }
                });
        });
    }
}
