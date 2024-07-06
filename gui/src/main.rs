use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use eframe::egui::{self, FontData, FontDefinitions};
use gui::{
    signals::{SignalProcessor, UpdateSignal},
    state::State,
};

fn main() {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("Mead", opts, Box::new(|cc| Ok(Box::new(MeadApp::new(cc))))).unwrap();
}

struct MeadApp {
    signal_tx: Sender<UpdateSignal>,
    state: Arc<Mutex<State>>,
}

impl MeadApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let ctx = cc.egui_ctx.clone();
        let (signal_tx, signal_rx) = mpsc::channel();
        let signal_proc = SignalProcessor::new(signal_rx);
        let state = Arc::new(Mutex::new(State::new()));

        let state_clone = state.clone();

        thread::spawn(move || {
            signal_proc.start(&ctx, state_clone);
        });

        Self { signal_tx, state }
    }
}

impl eframe::App for MeadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click").clicked() {
                self.signal_tx.send(UpdateSignal::AllProgramInfo).unwrap();
            }

            let s = self.state.lock().unwrap();

            for info in s.prog_infos.iter() {
                ui.label(&info.name);
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
