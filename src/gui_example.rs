use egui::{Align2, Context};

pub fn GUI(ui: &Context) {
    egui::Window::new("Buenos dias!")
        .show(&ui, |mut ui| {
            if ui.add(egui::Button::new("Click me")).clicked() {
                println!("PRESSED")
            }

            ui.label("Slider");
            let mut age = 0;
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            ui.end_row();
        });
}