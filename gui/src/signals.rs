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
            match self.rx.recv() {
                Ok(UpdateSignal::AllProgramInfo) => match api_client::get_prog_infos() {
                    Ok(resp) => {
                        let mut s = state.lock().unwrap();
                        s.prog_infos = resp;
                    }
                    Err(e) => println!("{e}"),
                },
                _ => {
                    println!("Exiting SignalProcessor thread");
                    return;
                }
            };

            ctx.request_repaint();
        }
    }
}
