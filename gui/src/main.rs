use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use eframe::egui;
use gui::signals::UpdateSignal;

fn main() {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("Mead", opts, Box::new(|cc| Ok(Box::new(MeadApp::new(cc))))).unwrap();
}

struct MeadApp {
    signal_tx: Sender<UpdateSignal>,
}

impl MeadApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = cc.egui_ctx.clone();
        let (signal_tx, signal_rx) = mpsc::channel();
        thread::spawn(move || loop {
            let signal = signal_rx.recv().unwrap();
            match signal {
                UpdateSignal::AllProgramInfo => {
                    match ureq::get("http://localhost:3000/ping").call() {
                        Ok(r) => println!("{}", r.into_string().unwrap()),
                        Err(e) => println!("{}", e.to_string()),
                    };
                }
            }
            ctx.request_repaint();
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
