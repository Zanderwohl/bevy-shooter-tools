use bevy::prelude::*;
use bevy::reflect::erased_serde::{Error, Serializer};
use bevy_egui::egui;
use bevy_egui::egui::{Context, Slider, SliderClamping, Ui};
use bevy_egui::egui::style::HandleShape;
use serde::{Deserialize, Serialize};
use crate::common::PointResolutionError;
use crate::editor::editable::EditorObject;
use crate::get;

#[derive(Serialize, Deserialize)]
pub struct GlobalPoint {
    location: Vec3,
}

#[typetag::serde(name = "global_point")]
impl EditorObject for GlobalPoint {
    fn get_point(&self, key: &str) -> Result<Vec3, PointResolutionError> {
        Ok(self.location)
    }

    fn editor_ui(&mut self, ctx: &mut Context) {
        egui::Window::new(self.type_name()).show(ctx, |ui| {
            ui.add(Slider::new(&mut self.location.x, -10.0..=10.0)
                .text("x")
                .clamping(SliderClamping::Never)
                .handle_shape(HandleShape::Rect { aspect_ratio: 1.0 })
            );
            ui.add(Slider::new(&mut self.location.y, -10.0..=10.0)
                .text("y")
                .clamping(SliderClamping::Never)
                .handle_shape(HandleShape::Rect { aspect_ratio: 1.0 })
            );
            ui.add(Slider::new(&mut self.location.z, -10.0..=10.0)
                .text("z")
                .clamping(SliderClamping::Never)
                .handle_shape(HandleShape::Rect { aspect_ratio: 1.0 })
            );
        });
    }

    fn type_name(&self) -> String {
        get!("editor.actions.global_point.title")
    }
    
    fn debug_gizmos(&self, gizmos: &mut Gizmos) {
        gizmos.sphere(Isometry3d::from_translation(self.location), 0.2, Color::srgb_u8(0, 255, 0));
    }
}

impl GlobalPoint {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            location: Vec3::new(x, y, z),
        }
    }
}
