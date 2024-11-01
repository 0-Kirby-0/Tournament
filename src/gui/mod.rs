use eframe::egui::TextureOptions;

use crate::simulation;

pub struct SimulationApp {
    simulation: simulation::Simulation,
}

impl SimulationApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let simulation = simulation::Simulation::new_pub();
        SimulationApp { simulation }
    }
}

impl eframe::App for SimulationApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(eframe::egui::Visuals::dark());
        let color_image = eframe::egui::ColorImage::from_rgb(
            [1000, 1000],
            &self
                .simulation
                .current()
                .as_byte_stream()
                .collect::<Vec<_>>(),
        );
        let texture = ctx.load_texture("blegh", color_image, TextureOptions::default());

        eframe::egui::CentralPanel::default().show(ctx, |ui| ui.image(&texture));
    }
}
