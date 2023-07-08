use glue_editor::*;

fn main() {
    glue_app("Glue Code Editor", Box::new(|_cc| Box::<FullEditor>::default()))
}

#[derive(Default)]
pub struct FullEditor {}

impl eframe::App for FullEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("Hi").min_width(200.0).max_width(400.0).show_inside(ui, |ui| {
                ui.heading("Left panel")
            });

            ui.heading("My egui Application");
        });
    }
}