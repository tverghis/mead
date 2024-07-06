use std::sync::mpsc::Receiver;

use eframe::egui;
use ureq::Response;

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

    pub fn start(&self, ctx: &egui::Context) {
        loop {
            // TODO: this will panic if tx is dropped, which happens when the main thread ends.
            // This isn't really a big problem because if the main thread ends, the GUI has exited anyway.
            // Would be nice to clean this up, though.
            let signal = self.rx.recv().unwrap();

            match signal {
                UpdateSignal::AllProgramInfo => match send_req() {
                    Ok(r) => println!("{}", r.into_string().unwrap()),
                    Err(e) => println!("{e}"),
                },
            };

            ctx.request_repaint();
        }
    }
}

fn send_req() -> Result<Response, ureq::Error> {
    ureq::get("http://localhost:3000/ping").call()
}
