

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    _padding: u32,
    pub color: [f32; 3],
    _padding2: u32,
} 

impl LightUniform {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        LightUniform {
            position,
            _padding: 0,
            color,
            _padding2: 0,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Light")
            .show(ctx, |ui| {
                ui.collapsing("Transform", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.add(egui::DragValue::new(&mut self.position[0]).prefix("x: "));
                        ui.add(egui::DragValue::new(&mut self.position[1]).prefix("y: "));
                        ui.add(egui::DragValue::new(&mut self.position[2]).prefix("z: "));
                    });
                });
            });
    }
}

impl Default for LightUniform {
    fn default() -> Self {
        LightUniform {
            position: [0.0, 0.0, 0.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        }
    }
}