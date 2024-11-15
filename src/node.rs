use eframe::egui;

pub struct Resource {
    name: String,
    image: Box<std::path::Path>,
    map_quantity: Option<u64>,
}

pub struct Machine {
    name: String,
    image: Box<std::path::Path>,
    power: Option<u64>,
}

pub struct Recipe {
    name: String,
    machine: Option<Machine>,
    inputs: Option<Vec<(Resource, f64)>>,
    output: Option<(Resource, f64)>,
    byproducts: Option<Vec<(Resource, f64)>>,
    power: Option<u32>,
    time: Option<f32>
}

pub struct Node {
    recipe: Recipe,
    position: egui::Pos2,
    factory: Option<u32>,
    area: egui::containers::Area,
    frame: Option<egui::Frame>,
}