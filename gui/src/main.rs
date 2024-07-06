use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use eframe::egui::{self};
use gui::signals::UpdateSignal;

fn main() {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("Mead", opts, Box::new(|cc| Ok(Box::new(MeadApp::new(cc))))).unwrap();
}

struct MeadApp {
    signal_tx: Sender<UpdateSignal>,
    counter: Arc<AtomicU64>,
}

impl MeadApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let counter: Arc<AtomicU64> = Arc::new(0.into());
        let counter_clone = counter.clone();
        let (signal_tx, signal_rx) = mpsc::channel();
        thread::spawn(move || loop {
            let signal = signal_rx.recv().unwrap();
            match signal {
                UpdateSignal::AllProgramInfo => {
                    println!("Signal received");
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        Self { signal_tx, counter }
    }
}

impl eframe::App for MeadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click").clicked() {
                self.signal_tx.send(UpdateSignal::AllProgramInfo).unwrap();
            }
            ui.label(self.counter.load(Ordering::SeqCst).to_string());
        });
    }
}
