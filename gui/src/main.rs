use eframe::egui;

fn main() {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("Mead", opts, Box::new(|cc| Ok(Box::new(MeadApp::new(cc))))).unwrap();
}

struct MeadApp;

impl MeadApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for MeadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Mead!");
        });
    }
}
