use eframe::egui;
use egui::{emath::TSTransform, TextWrapMode};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Universal Factory Tool",
        options,
        Box::new(|cc| { Ok(Box::<UniversalFactoryTool>::default()) }),
    )
}

struct UniversalFactoryTool {
    transform: TSTransform,
    drag_value: f32,
}

impl Default for UniversalFactoryTool {
    fn default() -> Self {
        Self {
            transform: TSTransform { scaling: 1.0, translation: egui::Vec2 { x: 0.0, y: 0.0 } },
            drag_value: 0.0
        }
    }
}

impl eframe::App for UniversalFactoryTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Universal Factory Tool");
            ui.label("panel with stuff here")
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (id, rect) = ui.allocate_space(ui.available_size());
            let area = ui.interact(rect, id, egui::Sense::click_and_drag());
            if area.dragged() {
                self.transform.translation += area.drag_delta();
            }

            let transform = TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;
            
            for (i, (pos, callback)) in [
                (
                    egui::Pos2::new(0.0, 0.0),
                    Box::new(|ui: &mut egui::Ui, _: &mut Self| ui.button("top left!"))
                        as Box<dyn Fn(&mut egui::Ui, &mut Self) -> egui::Response>,
                ),
                (
                    egui::Pos2::new(0.0, 120.0),
                    Box::new(|ui: &mut egui::Ui, _| ui.button("bottom left?")),
                ),
                (
                    egui::Pos2::new(120.0, 120.0),
                    Box::new(|ui: &mut egui::Ui, _| ui.button("right bottom :D")),
                ),
                (
                    egui::Pos2::new(120.0, 0.0),
                    Box::new(|ui: &mut egui::Ui, _| ui.button("right top ):")),
                ),
            ]
            .into_iter()
            .enumerate()
            {
                let window_layer = ui.layer_id();
                let id = egui::Area::new(id.with(("subarea", i)))
                    .default_pos(pos)
                    .order(egui::Order::Middle)
                    .constrain(false)
                    .show(ui.ctx(), |ui| {
                        ui.set_clip_rect(transform.inverse() * rect);
                        egui::Frame::default()
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::same(8.0))
                            .stroke(ui.ctx().style().visuals.window_stroke)
                            .fill(ui.style().visuals.panel_fill)
                            .show(ui, |ui| {
                                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                                callback(ui, self)
                            });
                    })
                    .response
                    .layer_id;
                ui.ctx().set_transform_layer(id, transform);
                ui.ctx().set_sublayer(window_layer, id);
            }
        });
    }
}