// Authorisation Required - Main File
// Author: Arsalan Kazmi (AeriaVelocity) <sonicspeed848@gmail.com>

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release mode

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Authorisation Required",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Game>::default())
        }),
    )
}

struct Game {
    input: String,
    output: Vec<String>
}

impl Default for Game {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: Vec::new()
        }
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Authorisation Required");

            // Terminal-like UI
            ui.group(|ui| {
                for line in &self.output {
                    ui.label(line);
                }

                ui.horizontal(|ui| {
                    ui.label("authreq-sh$ ");
                    ui.text_edit_singleline(&mut self.input);
                    // TODO Implement the `Enter` key later
                });
            });
        });
    }
}

impl Game {
    fn handle_input(&mut self) {
        self.output.push(
            match self.input.trim() {
                "hello" => String::from("Hello, world!"),
                _ => format!("Unknown command `{}`", self.input),
            }
        );
        self.input.clear();
    }
}