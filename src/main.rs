use eframe::egui; // Import necessary parts of eframe and egui
use egui_extras;
// use chrono;
use jiff::civil::Date;

// The main function where our program starts
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Demo",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// This struct holds the data (state) for our application.
#[derive(Default)]
struct MyApp {
    label: String,
    value: f32,
    date: Date,
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
        ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        ui.add(egui_extras::DatePickerButton::new(&mut self.date));
    }
}
