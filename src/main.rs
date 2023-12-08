#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// use std::io;
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


// fn main() -> io::Result<()> {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input)?;
//     println!("{} has {} characters", input.trim(), input.trim().chars().count());
//     Ok(())
// }


fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}