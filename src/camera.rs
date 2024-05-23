use cgmath::*;
use instant::Duration;
use std::f32::consts::FRAC_PI_2;
use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::keyboard::{KeyCode, PhysicalKey};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }
}

#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

#[derive(Debug)]
pub struct CameraController {
    pub camera: Camera,
    pub projection: Projection,
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32, camera: Camera, projection: Projection) -> Self {
        Self {
            camera,
            projection,
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, event: &KeyEvent) -> bool {
        let amount = if event.state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        if let PhysicalKey::Code(keycode) = event.physical_key {
            match keycode {
                KeyCode::KeyW | KeyCode::ArrowUp => {
                    self.amount_forward = amount;
                    true
                }
                KeyCode::KeyS | KeyCode::ArrowDown => {
                    self.amount_backward = amount;
                    true
                }
                KeyCode::KeyA | KeyCode::ArrowLeft => {
                    self.amount_left = amount;
                    true
                }
                KeyCode::KeyD | KeyCode::ArrowRight => {
                    self.amount_right = amount;
                    true
                }
                KeyCode::Space => {
                    self.amount_up = amount;
                    true
                }
                KeyCode::ShiftLeft => {
                    self.amount_down = amount;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, dt: Duration) {
        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = self.camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        let mut resultant = Vector3::zero();
        resultant += forward * (self.amount_forward - self.amount_backward);
        resultant += right * (self.amount_right - self.amount_left);

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = self.camera.pitch.0.sin_cos();
        let scrollward =
            Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        resultant += scrollward * self.scroll;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        resultant.y += self.amount_up - self.amount_down;

        if resultant != Vector3::zero() {
            self.camera.position += resultant.normalize() * self.speed * dt;
        }

        // Rotate
        self.camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        self.camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non-cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if self.camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            self.camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if self.camera.pitch > Rad(SAFE_FRAC_PI_2) {
            self.camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }

    pub fn ui(&mut self, ui: &egui::Context) {
        egui::Window::new("Camera Controller").show(ui, |ui| {
            Self::ui_component(ui, |ui| {
                ui.label("Transform");
                ui.horizontal(|ui| {
                    ui.label("Position:");
                    ui.add(egui::DragValue::new(&mut self.camera.position.x).prefix("x: "));
                    ui.add(egui::DragValue::new(&mut self.camera.position.y).prefix("y: "));
                    ui.add(egui::DragValue::new(&mut self.camera.position.z).prefix("z: "));
                });
                ui.horizontal(|ui| {
                    ui.label("Rotations:");
                    ui.add(egui::DragValue::new(&mut self.camera.yaw.0).prefix("yaw: "));
                    ui.add(egui::DragValue::new(&mut self.camera.pitch.0).prefix("pitch: "));
                });
            });

            Self::ui_component(ui, |ui| {
                ui.label("Projection");
                ui.horizontal(|ui| {
                    ui.label("Aspect:");
                    ui.add(
                        egui::Slider::new(&mut self.projection.aspect, 1.0..=2.0).text("Aspect"),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Fovy:");
                    ui.add(
                        egui::Slider::new(&mut self.projection.fovy.0, 0.1..=FRAC_PI_2)
                            .text("Fovy"),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Znear:");
                    ui.add(
                        egui::Slider::new(&mut self.projection.znear, 0.1..=100.0).text("Znear"),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Zfar:");
                    ui.add(
                        egui::Slider::new(&mut self.projection.zfar, 100.0..=1000.0).text("Zfar"),
                    );
                });
            });

            Self::ui_component(ui, |ui| {
                ui.label("Movement");
                ui.add(egui::Slider::new(&mut self.speed, 0.0..=100.0).text("Speed"));
                ui.add(egui::Slider::new(&mut self.sensitivity, 0.0..=2.0).text("Sensitivity"));
            });
        });
    }

    pub fn ui_component(ui: &mut egui::Ui, run_component: impl FnOnce(&mut egui::Ui)) {
        ui.group(|ui| ui.vertical_centered(run_component));
    }
}
