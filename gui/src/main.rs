use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use eframe::egui::{self};

fn main() {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("Mead", opts, Box::new(|cc| Ok(Box::new(MeadApp::new(cc))))).unwrap();
}

struct MeadApp {
    counter: Arc<AtomicU64>,
}

impl MeadApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let counter: Arc<AtomicU64> = Arc::new(0.into());
        let ctx_clone = cc.egui_ctx.clone();
        let counter_clone = counter.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));
            counter_clone.fetch_add(1, Ordering::SeqCst);
            ctx_clone.request_repaint();
        });
        Self {
            counter: counter.clone(),
        }
    }
}

impl eframe::App for MeadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Mead!");
            ui.label(format!("{}", self.counter.load(Ordering::SeqCst)));
        });
    }
}
