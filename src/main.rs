use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Universal Factory Tool",
        options,
        Box::new(|cc| { Ok(Box::<UniversalFactoryTool>::default()) }),
    )
}

struct UniversalFactoryTool {}

impl Default for UniversalFactoryTool {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for UniversalFactoryTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Universal Factory Tool");
        });
    }
}