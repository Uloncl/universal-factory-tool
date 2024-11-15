use eframe::egui;
use egui::{emath::TSTransform, Shape, epaint::{ColorMode, PathStroke}, Color32, Painter, TextWrapMode};

pub mod node;

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
    game: String,
    resources: Option<Vec<node::Resource>>,
    machines: Option<Vec<node::Machine>>,
    recipes: Option<Vec<node::Recipe>>,
}

impl Default for UniversalFactoryTool {
    fn default() -> Self {
        Self {
            transform: TSTransform { scaling: 1.0, translation: egui::Vec2 { x: 0.0, y: 0.0 } },
            drag_value: 0.0,
            game: "".to_string(),
            resources: None,
            machines: None,
            recipes: None
        }
    }
}

impl eframe::App for UniversalFactoryTool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Universal Factory Tool");
            ui.label("panel with stuff here");
        });
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.button("like these");
                ui.button("for buttons");
                ui.button("the top panel");
                ui.button("this is");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (id, rect) = ui.allocate_space(ui.available_size());
            let area = ui.interact(rect, id, egui::Sense::click_and_drag());
            if area.dragged() {
                self.transform.translation += area.drag_delta();
            }

            let transform = TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

            for (i, (display, pos, callback)) in [
                (
                    false,
                    egui::Pos2::new(60.0, 60.0),
                    Box::new(|ui: &mut egui::Ui, _: &mut Self| {
                        let painter = ui.painter();
                        for y in (-69420..69420).step_by(100) {
                            painter.hline(-69420.0..=69420.0, y as f32, (1.0, Color32::from_rgb(32, 32, 32)));
                        }
                        for x in (-69420..69420).step_by(100) {
                            painter.vline(x as f32, -69420.0..=69420.0, (1.0, Color32::from_rgb(32, 32, 32)));
                        }
                    })
                    as Box<dyn Fn(&mut egui::Ui, &mut Self) -> ()>,
                ),
                (
                    true,
                    egui::Pos2::new(0.0, 0.0),
                    Box::new(|ui: &mut egui::Ui, _| { ui.selectable_label(false, "top left!"); }),
                ),
            ]
            .into_iter()
            .enumerate()
            {
                let window_layer = ui.layer_id();
                let node = egui::Area::new(id.with(("subarea", i)))
                    .default_pos(pos)
                    .order(egui::Order::Middle)
                    .constrain(false)
                    .show(ui.ctx(), |ui| {
                        ui.set_clip_rect(transform.inverse() * rect);
                        if display {
                            egui::Frame::default().show(ui, |ui| {
                                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                                callback(ui, self)
                            });
                        } else {
                            callback(ui, self)
                        }
                    });
                println!("{:?}", node.response.drag_delta());
                ui.ctx().set_transform_layer(node.response.layer_id, transform);
                ui.ctx().set_sublayer(window_layer, node.response.layer_id);
            }
        });
    }
}