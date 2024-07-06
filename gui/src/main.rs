use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use eframe::egui::{self, FontData, FontDefinitions};
use gui::signals::{SignalProcessor, UpdateSignal};

fn main() {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("Mead", opts, Box::new(|cc| Ok(Box::new(MeadApp::new(cc))))).unwrap();
}

struct MeadApp {
    signal_tx: Sender<UpdateSignal>,
}

impl MeadApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let ctx = cc.egui_ctx.clone();
        let (signal_tx, signal_rx) = mpsc::channel();
        let signal_proc = SignalProcessor::new(signal_rx);
        thread::spawn(move || {
            signal_proc.start(&ctx);
        });
        Self { signal_tx }
    }
}

impl eframe::App for MeadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click").clicked() {
                self.signal_tx.send(UpdateSignal::AllProgramInfo).unwrap();
            }
        });
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let proggy_ttf = include_bytes!("../assets/ProggyClean.ttf");
    let font_name = "proggy_clear";

    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert(font_name.into(), FontData::from_static(proggy_ttf));
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, font_name.into());
    ctx.set_fonts(fonts);
}
