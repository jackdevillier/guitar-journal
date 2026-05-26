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
    id: u64,
    artist: String,
    song: String
}

// This struct holds the data (state) for our application.
#[derive(Default)]
struct MyApp {
    date: Date,
    window_open: bool,
    time_played_minutes: u8,
    time_played_hours: u8,
    songs: Vec<Song>,
    next_id: u64,
    chords: String,
    techniques: String,
}

impl MyApp {
    fn add_song(&mut self, id: u64) {
        self.songs.push(Song {
            id: id,
            artist: "".to_string(),
            song: "".to_string()
        });
        self.next_id += 1;
    }

    fn remove_song(&mut self, id: u64) {
        self.songs.retain(|s| s.id != id);
    }

    // get the current date
    fn get_date(&mut self) -> Date {
        let chrono_date = chrono::Local::now();
        let ret = Date::new(chrono_date.year().try_into().unwrap(), chrono_date.month().try_into().unwrap(), chrono_date.day().try_into().unwrap());
        return ret.unwrap();
    }

}

// We implement the `eframe::App` trait for our struct.
impl eframe::App for MyApp {
    // The `update` function is called repeatedly, once per frame.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Guitar Journal");
        // ui.horizontal(|ui| {
        //     ui.label("Write something: ");
        //     ui.text_edit_singleline(&mut self.label);
        // });
        // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
        // if ui.button("Increment").clicked() {
        //     self.value += 1.0;
        // }

        let mut add_clicked = false;
        let mut id_to_remove = None;

        // FORM FOR NEW JOURNAL ENTRIES
        egui::Window::new("New Entry").open(&mut self.window_open).show(ui.ctx(), |ui| {
            // ui.heading("New Entry");
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

            for song in &mut self.songs {
                ui.horizontal(|ui| {
                    // artist entry
                    ui.label("Artist: ");
                    ui.text_edit_singleline(&mut song.artist);
                    
                    // song entry
                    ui.label("Song: ");
                    ui.text_edit_singleline(&mut song.song);
    
                    // remove button
                    if ui.button("-").clicked() {
                        id_to_remove = Some(song.id);
                    }
                });
            }
            if ui.button("Add Song").clicked() {
                add_clicked = true;
            }

            // chords
            ui.horizontal(|ui| {
                ui.label("Chords Used: ");
                ui.text_edit_singleline(&mut self.chords)
            });

            // guitar techniques: arp, plucking, hammer on
            ui.horizontal(|ui| {
                ui.label("Techniques Used: ");
                ui.text_edit_singleline(&mut self.techniques)
            });

            // new ideas for laters: scale shapes, triads, tuning


            if ui.button("Save Entry").clicked() {
                // do save entry stuff with serde
            }
        });

        if add_clicked {
            self.add_song(self.next_id);
        }

        if let Some(id) = id_to_remove {
            self.remove_song(id);
        }
        
        if ui.button("New Entry").clicked() {
            self.window_open = !self.window_open;
            self.date = self.get_date();
            self.time_played_minutes = 0;
            self.time_played_hours = 0;
            self.songs.clear();
            self.chords = "".to_string();
        }
        // ui.label(format!("Hello '{}', value: {}", self.label, self.value));
    }

}

