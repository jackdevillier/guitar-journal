use std::any::Any;

use eframe::egui; // Import necessary parts of eframe and egui
use egui_extras;
use chrono::{self, Datelike};
use jiff::civil::Date;
use serde::{Serialize, Deserialize};

// The main function where our program starts
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Demo",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct Song {
    id: u32,
    artist: String,
    song: String
}

// This struct holds the data (state) for our application.
#[derive(Default)]
struct MyApp {
    label: String,
    value: f32,
    date: Date,
    window_open: bool,
    time_played_minutes: u8,
    time_played_hours: u8,
    songs: Vec<Song>,
}

// We implement the `eframe::App` trait for our struct.
impl eframe::App for MyApp {
    // The `update` function is called repeatedly, once per frame.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            ui.label("Write something: ");
            ui.text_edit_singleline(&mut self.label);
        });
        ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
            self.value += 1.0;
        }

        // FORM FOR NEW JOURNAL ENTRIES
        egui::Window::new("New Entry").open(&mut self.window_open).show(ui.ctx(), |ui| {
            // date
            ui.horizontal(|ui| {
                ui.label("Date of Entry: ");
                
                ui.add(egui_extras::DatePickerButton::new(&mut self.date));
            });
            
            // time practiced
            ui.horizontal(|ui| {
                ui.label("Time practiced: ");
                // hours
                ui.add(egui::DragValue::new(&mut self.time_played_hours).range(0..=24).speed(0.05));
                ui.label("hr");
                // minutes
                ui.add(egui::DragValue::new(&mut self.time_played_minutes).range(0..=60).speed(0.2));
                ui.label("min");
            });
            
            // songs
            ui.label("Song(s) Played: ");
            let mut counter = 0;
            ui.horizontal(|ui| {
                // artist entry

                // song entry

                // remove button
                if ui.button("-").clicked() {

                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {

                    let _ = self.songs.push_mut(Song {id: counter, artist: "".to_string(), song: "".to_string()});
                }
            });


            // chords


            // guitar techniques: arp, plucking, hammer on


            // new ideas for laters: scale shapes, triads, tuning

        });
        if ui.button("New Entry").clicked() {
            self.date = get_date();
            self.time_played_minutes = 0;
            self.time_played_hours = 0;
            self.window_open = true;
        }
        ui.label(format!("Hello '{}', value: {}", self.label, self.value));
    }
}

fn get_date() -> Date {
    let chrono_date = chrono::Local::now();
    let ret = Date::new(chrono_date.year().try_into().unwrap(), chrono_date.month().try_into().unwrap(), chrono_date.day().try_into().unwrap());
    return ret.unwrap();
}
