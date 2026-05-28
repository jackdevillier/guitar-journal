use eframe::egui; // Import necessary parts of eframe and egui
use egui_extras;
use chrono::{self, Datelike};
use jiff::civil::Date;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::{File, OpenOptions, create_dir, exists, remove_dir};
use std::io::{BufReader, BufWriter, Write};

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

#[derive(Serialize, Deserialize, Debug)]
struct GuitarEntry {
    date_day: i8,
    date_month: i8,
    date_year: i16,
    time_played_minutes: u8,
    time_played_hours: u8,
    songs: Vec<String>,
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
        self.next_id -= 1;
        let mut count = 0;
        for song in &mut self.songs {
            song.id = count;
            count += 1;
        }
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
                // check if the entry has a valid length of time practiced
                // do save entry stuff with serde
                let mut ser_songs: Vec<String> = Vec::new();
                for song in &mut self.songs {
                    ser_songs.push(format!("{}|{}", song.artist, song.song));
                }
                // create guitar entry struct
                let entry = GuitarEntry {
                    date_day: self.date.day(),
                    date_month: self.date.month(),
                    date_year: self.date.year(),
                    time_played_minutes: self.time_played_minutes,
                    time_played_hours: self.time_played_hours,
                    songs: ser_songs,
                    chords: self.chords.clone(),
                    techniques: self.techniques.clone()
                };

                // todo: check if file to insert entry exists
                /* 
                 * check if file exists
                 * if not, create a new file
                 *     make a file for each day but keep them in folders
                 *     ../entry_data/YYYY/MM/entries_MM_DD_YYYY.jsonl
                 */

                let entry_path = "C:/Users/jdevi/local_projects/guitar-journal/entry_data";

                // check year folder exists; make one if not
                if !exists(format!("{}/{}", entry_path, entry.date_year)).unwrap() {
                    let _ = create_dir(format!("{}/{}", entry_path, entry.date_year));
                }

                let json_path = format!("{}/{}/{}", entry_path, entry.date_year, entry.date_month);
                // check month folder exists; make one if not
                if !exists(&json_path).unwrap() {
                    let _ = create_dir(&json_path);
                }
                // check day json file exists; make one if not
                let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(format!("{}/entries_{}_{}_{}.jsonl", json_path, entry.date_month, entry.date_day, entry.date_year))
                    .expect("Failed to open the file");

                let mut writer = BufWriter::new(file);

                let json_str = serde_json::to_string(&entry).unwrap();

                let _ = serde_json::to_writer(&mut writer, &json_str).unwrap();
                let _ = writer.write_all(b"\n").unwrap();

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

