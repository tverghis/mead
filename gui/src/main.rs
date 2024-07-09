use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

use eframe::egui::{
    self, Color32, FontData, FontDefinitions, FontFamily, FontId, Rounding, ScrollArea, TextStyle,
};
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
    selected_insn_idx: Option<usize>,
}

impl MeadApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        setup_widget_styles(&cc.egui_ctx);

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
            selected_insn_idx: None,
        }
    }
}

impl eframe::App for MeadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("browser_actions_panel").show(ctx, |ui| {
            if ui.button("Refresh").clicked() {
                self.signal_tx.send(UpdateSignal::AllProgramInfo).unwrap();
            }
        });
        egui::SidePanel::left("browser_panel").show(ctx, |ui| {
            let s = self.state.lock().unwrap();

            for info in s.prog_infos.iter() {
                let label_val = match info.name.as_str() {
                    "" => &info.tag,
                    _ => &info.name,
                };
                if ui
                    .selectable_label(
                        self.selected_prog_id == Some(info.id),
                        format!("{}: {}", info.id, label_val),
                    )
                    .clicked()
                {
                    self.selected_prog_id = Some(info.id);
                    self.selected_insn_idx = None;
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let xlated_insns = {
                    let s = self.state.lock().unwrap();
                    s.prog_infos
                        .iter()
                        .find(|info| Some(info.id) == self.selected_prog_id)
                        .map(|info| info.xlated_insns.clone())
                };

                match xlated_insns {
                    Some(insns) => {
                        let insns_as_hex_str = insns
                            .chunks(8)
                            .map(|w| {
                                w.iter()
                                    .map(|b| format!("{b:02x}"))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .collect::<Vec<_>>();
                        for (idx, i) in insns_as_hex_str.iter().enumerate() {
                            if ui
                                .selectable_label(Some(idx) == self.selected_insn_idx, i)
                                .clicked()
                            {
                                self.selected_insn_idx = Some(idx);
                            }
                        }
                    }
                    None => {
                        ui.label("No instructions");
                    }
                };
            });
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
        (TextStyle::Body, FontId::new(13.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(13.0, FontFamily::Proportional),
        ),
    ]
    .into();
    style.visuals.override_text_color = Some(Color32::WHITE);
    ctx.set_style(style);
}

fn setup_widget_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.widgets.inactive.rounding = Rounding::ZERO;
    style.visuals.widgets.active.rounding = Rounding::ZERO;
    style.visuals.widgets.hovered.rounding = Rounding::ZERO;
    style.visuals.widgets.open.rounding = Rounding::ZERO;
    ctx.set_style(style);
}
