use eframe::egui; // Import necessary parts of eframe and egui
use egui_extras;
use chrono::{self, Datelike};
use jiff::civil::Date;
use serde::{Serialize, Deserialize};
use serde_json;
use std::env;
use std::fs::{File, OpenOptions, create_dir, exists, remove_dir};
use std::io::{BufRead, BufReader, BufWriter, Write};

/* 
 * TODO:
 * bufreader
 * delete entries
 * create view 
 * 
 */

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

struct DisplayEntry {
    id: u64,
    entry: GuitarEntry,
}

// This struct holds the data (state) for our application.
#[derive(Default)]
struct MyApp {
    defaults_assigned: bool,
    date: Date,
    new_entry_window_open: bool,
    time_played_minutes: u8,
    time_played_hours: u8,
    songs: Vec<Song>,
    next_id: u64,
    chords: String,
    techniques: String,
    display: EntryListState,
}

#[derive(Default)]
struct EntryListState {
    window_open: bool,
    date: Date,
    display_text: String,
    entries: Vec<GuitarEntry>,
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
    // fn song_str_to_struct(&mut self, song: String) -> Song {
    //     let mut song_parts = song.split("|");
    //     let mut res: Song = Song {id: 0, artist: "".to_string(), song: "".to_string()};
    //     res.artist = song_parts.next().unwrap().to_string();
    //     res.song = song_parts.next().unwrap().to_string();
    //     return res;
    // }

    fn entry_to_string(&mut self, entry: GuitarEntry, with_date: bool) -> String {
        let mut res: String = String::from("");
        if with_date {
            res += &format!("{}/{}/{}\n", entry.date_month, entry.date_day, entry.date_year);
        }
        /*
            chords: String,
            techniques: String,
         */
        res += &format!("Time Played: {} hours, {} minutes\n", entry.time_played_hours, entry.time_played_minutes);

        // add songs
        if entry.songs.len() > 0 {
            res += &format!("{} songs:\n", entry.songs.len());
            for song in entry.songs {
                let mut song_parts = song.split("|");
                res += &format!(" • {} - {}\n", song_parts.next().unwrap().to_string(), song_parts.next().unwrap().to_string());
            }
        }

        // chords
        if entry.chords.len() > 0 {
            res += &format!("Chords Practiced: {}\n", entry.chords);
        }

        // techniques
        if entry.techniques.len() > 0 {
            res += &format!("Techniques Used: {}\n", entry.techniques);
        }
        return res;
    }

    fn get_entries_for_day(&mut self, date: Date) -> Vec<GuitarEntry> {
        let mut res: Vec<GuitarEntry> = Vec::new();
        let curr_path_buf = env::current_dir().unwrap();
        let entry_path = curr_path_buf.as_os_str().to_str().unwrap().to_owned() + "/entry_data";
        // check year folder exists
        if exists(format!("{}/{}", entry_path, date.year())).unwrap() {
            let json_path = format!("{}/{}/{}", entry_path, date.year(), date.month());
            // check month folder exists
            if exists(&json_path).unwrap() && exists(format!("{}/entries_{}_{}_{}.jsonl", json_path, date.month(), date.day(), date.year())).unwrap() {
                let file = File::open(format!("{}/entries_{}_{}_{}.jsonl", json_path, date.month(), date.day(), date.year())).expect("File does not exist");
                let mut reader = BufReader::new(file);
                let mut dest: String = String::new();
                let mut read_size = reader.read_line(&mut dest).unwrap();
                // let entry: GuitarEntry = serde_json::from_reader(reader).unwrap();

                while read_size > 0 {
                    // println!("{:?}", read_size);
                    let inner: String = serde_json::from_str(&dest).unwrap();
                    let entry: GuitarEntry = serde_json::from_str(&inner).unwrap();
                    // println!("{}/{}/{}, {}hr{}min", entry.date_month, entry.date_day, entry.date_year, entry.time_played_hours, entry.time_played_minutes);
                    res.push(entry);
                    dest.clear();
                    read_size = reader.read_line(&mut dest).unwrap();
                }
                return res;
            }
        }

        return res;
    }

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
        if self.defaults_assigned == false {
            self.display.date = self.get_date();
            self.defaults_assigned = true;
        }
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
        let mut save_clicked = false;

        // FORM FOR NEW JOURNAL ENTRIES
        egui::Window::new("New Entry").open(&mut self.new_entry_window_open).show(ui.ctx(), |ui| {
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

                let curr_path_buf = env::current_dir().unwrap();
                // println!("{}", curr_path_buf.as_os_str().to_str().unwrap());

                let entry_path = curr_path_buf.as_os_str().to_str().unwrap().to_owned() + "/entry_data";

                // check entry_data folder exists; make one if not
                if !exists(format!("{}", entry_path)).unwrap() {
                    let _ = create_dir(format!("{}", entry_path));
                }
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
                save_clicked = true;
            }
        });
        
        if save_clicked {
            self.new_entry_window_open = !self.new_entry_window_open;
        }

        if add_clicked {
            self.add_song(self.next_id);
        }

        if let Some(id) = id_to_remove {
            self.remove_song(id);
        }
        
        if ui.button("New Entry").clicked() {
            self.new_entry_window_open = !self.new_entry_window_open;
            self.date = self.get_date();
            self.time_played_minutes = 0;
            self.time_played_hours = 0;
            self.songs.clear();
            self.chords.clear();
            self.techniques.clear();
        }

        // if ui.button("Fetch Today's Entries").clicked() {
        //     let date = self.get_date();
        //     let entry = self.get_entries_for_day(date);
        // }

        // ui.
        ui.heading("Entry Display");

        ui.add(egui_extras::DatePickerButton::new(&mut self.display.date));
        if ui.button("Display Entries").clicked() {
            self.display.display_text.clear();
            // read entries for chosen day
            let entries = self.get_entries_for_day(self.display.date);
            // populate text box beneath 
            let mut is_first_entry = true;
            for entry in entries {
                let e: String = self.entry_to_string(entry, is_first_entry);
                self.display.display_text.push_str(&e);
                is_first_entry = false;
            }
        }
        ui.label(format!("{}", &mut self.display.display_text));
    }
    
}

