use gui::app::Mead;

fn main() {
    let opts = eframe::NativeOptions::default();
    eframe::run_native("Mead", opts, Box::new(|cc| Ok(Box::new(Mead::new(cc))))).unwrap();
}
