use std::sync::{mpsc::Receiver, Arc, Mutex};

use eframe::egui;

use crate::api_client;
use crate::state::State;

pub enum UpdateSignal {
    AllProgramInfo,
}
pub struct SignalProcessor {
    rx: Receiver<UpdateSignal>,
}

impl SignalProcessor {
    pub fn new(rx: Receiver<UpdateSignal>) -> Self {
        Self { rx }
    }

    pub fn start(&self, ctx: &egui::Context, state: Arc<Mutex<State>>) {
        loop {
            // TODO: this will panic if tx is dropped, which happens when the main thread ends.
            // This isn't really a big problem because if the main thread ends, the GUI has exited anyway.
            // Would be nice to clean this up, though.
            let signal = self.rx.recv().unwrap();

            match signal {
                UpdateSignal::AllProgramInfo => match api_client::get_prog_infos() {
                    Ok(resp) => {
                        let mut s = state.lock().unwrap();
                        s.prog_infos = resp;
                    }
                    Err(e) => println!("{e}"),
                },
            };

            ctx.request_repaint();
        }
    }
}
