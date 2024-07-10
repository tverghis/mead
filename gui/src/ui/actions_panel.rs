use crate::app::MeadApp;
use crate::signals::UpdateSignal;
use eframe::egui::{self, Context};

pub struct ActionsPanel<'a> {
    state: &'a mut MeadApp,
}

impl<'a> ActionsPanel<'a> {
    pub fn new(state: &'a mut MeadApp) -> Self {
        Self { state }
    }
    pub fn render(self, ctx: &Context) {
        egui::TopBottomPanel::bottom("browser_actions_panel").show(ctx, |ui| {
            if ui.button("Refresh").clicked() {
                self.state
                    .signal_tx
                    .send(UpdateSignal::AllProgramInfo)
                    .unwrap();
            }
        });
    }
}
