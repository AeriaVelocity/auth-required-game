// Authorisation Required - Main File
// Author: Arsalan Kazmi (AeriaVelocity) <sonicspeed848@gmail.com>

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release mode

use eframe::egui;
use eframe::egui::Key;
use eframe::egui::FontId;
use eframe::egui::FontFamily::Monospace;
use eframe::egui::TextStyle::*;
use eframe::egui::Visuals;
use eframe::egui::Color32;

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
            let mut style = (*cc.egui_ctx.style()).clone();

            style.text_styles = [
                (Heading, FontId::new(16.0, Monospace)),
                (Body, FontId::new(14.0, Monospace)),
                (Small, FontId::new(10.0, Monospace)),
            ].into();

            style.visuals = Visuals {
                window_fill: Color32::from_rgb(0, 0, 0),
                override_text_color: Some(Color32::from_rgb(255, 255, 255)),
                ..Default::default()
            };

            cc.egui_ctx.set_style(style);

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
            // Okay, well, not really, eframe makes it look like an ordinary app
            // It doesn't really look like a terminal at all XD
            ui.group(|ui| {
                for line in &self.output {
                    ui.label(line);
                }

                ui.horizontal(|ui| {
                    ui.label("authreq-sh$ ");
                    let response = ui.text_edit_singleline(&mut self.input);
                    if ui.input(|i| i.key_pressed(Key::Enter)) {
                        self.handle_input();
                    }
                    response.request_focus();
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