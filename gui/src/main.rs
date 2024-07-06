use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use eframe::egui::{self, FontData, FontDefinitions, FontFamily, FontId, TextStyle};
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
    selected_prog_id: Option<u32>,
}

impl MeadApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let ctx = cc.egui_ctx.clone();

        let (signal_tx, signal_rx) = mpsc::channel();
        let signal_proc = SignalProcessor::new(signal_rx);

        let state_inner = State::new_from_api();

        let first_prog_id = state_inner.prog_infos.first().map(|i| i.id);

        let state = Arc::new(Mutex::new(state_inner));
        let state_clone = state.clone();

        thread::spawn(move || {
            signal_proc.start(&ctx, state_clone);
        });

        Self {
            signal_tx,
            state,
            selected_prog_id: first_prog_id,
        }
    }
}

impl eframe::App for MeadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let s = self.state.lock().unwrap();

            if ui.button("Click").clicked() {
                self.signal_tx.send(UpdateSignal::AllProgramInfo).unwrap();
            }

            for info in s.prog_infos.iter() {
                if ui
                    .selectable_label(
                        self.selected_prog_id == Some(info.id),
                        format!("{}: {}", info.id, info.name),
                    )
                    .clicked()
                {
                    self.selected_prog_id = Some(info.id);
                }
            }
        });
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let proggy_ttf = include_bytes!("../assets/ProggyVector-Regular.ttf");
    let font_name = "proggy";

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

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Body, FontId::new(9.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(9.0, FontFamily::Proportional),
        ),
    ]
    .into();
    ctx.set_style(style);
}
