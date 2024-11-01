#![allow(dead_code, unused_variables)]

mod colors;
mod gui;
mod simulation;

fn main() {
    eframe::run_native(
        "Simulation",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(gui::SimulationApp::new(cc)))),
    )
    .unwrap();
}
