#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

// Create a program that prompts for an input string and dis- plays output that shows the input string and the number of characters the string contains.

// verbs: prompts, displays, shows
// nouns: input string, output, number of characters

// Input: input string
// Process: Count the number of characters
// Output: input string, and number of characters

// Test
// Input: Homer
// Expected Output: Homer has 5 characters


fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut text = "".to_owned();

    eframe::run_simple_native("Text Counter", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Count the number of characters");

            let text_label = ui.label("Enter Text: ");
            ui.text_edit_singleline(&mut text)
                .labelled_by(text_label.id);
            
            if text.len() > 0 {
                ui.label(format!("{} has {} characters", text, text.chars().count()));
            };
        });
    })
}