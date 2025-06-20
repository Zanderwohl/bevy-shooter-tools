use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::{Context, Ui};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::common::PointResolutionError;
use crate::editor::editable::EditorObject;
use crate::get;

lazy_static! {
    static ref PLANE_CENTERS: [CuboidPoint; 6] = {
        [
            CuboidPoint::TopPlaneCenter,
            CuboidPoint::BottomPlaneCenter,
            CuboidPoint::RightPlaneCenter,
            CuboidPoint::LeftPlaneCenter,
            CuboidPoint::BackPlaneCenter,
            CuboidPoint::FrontPlaneCenter,
        ]
    };
}

#[derive(Clone, Copy)]
pub enum CuboidPoint {
    Centroid,
    TopPlaneCenter,
    FrontPlaneCenter,
    BottomPlaneCenter,
    BackPlaneCenter,
    LeftPlaneCenter,
    RightPlaneCenter,
    FrontBottomLeftCorner,
    FrontBottomRightCorner,
    FrontTopLeftCorner,
    FrontTopRightCorner,
    BackBottomLeftCorner,
    BackBottomRightCorner,
    BackTopLeftCorner,
    BackTopRightCorner,
    FrontTopEdgeCenter,
    FrontBottomEdgeCenter,
    FrontLeftEdgeCenter,
    FrontRightEdgeCenter,
    BackTopEdgeCenter,
    BackBottomEdgeCenter,
    BackLeftEdgeCenter,
    BackRightEdgeCenter,
    BottomLeftEdgeCenter,
    BottomRightEdgeCenter,
    TopLeftEdgeCenter,
    TopRightEdgeCenter,
}

impl CuboidPoint {
    pub fn value(&self) -> [f32; 3] { // Right-hand y-up: thumb x, index y, middle z
        match self {
            CuboidPoint::Centroid => [0.5, 0.5, 0.5],
            CuboidPoint::TopPlaneCenter => [0.5, 1.0, 0.5],
            CuboidPoint::FrontPlaneCenter => [0.0, 0.5, 0.5],
            CuboidPoint::BottomPlaneCenter => [0.5, 0.0, 0.5],
            CuboidPoint::BackPlaneCenter => [0.0, 0.5, 0.5],
            CuboidPoint::LeftPlaneCenter => [0.5, 0.5, 1.0],
            CuboidPoint::RightPlaneCenter => [0.5, 0.5, 0.0],
            CuboidPoint::FrontBottomLeftCorner => [0.0, 0.0, 1.0],
            CuboidPoint::FrontBottomRightCorner => [0.0, 0.0, 0.0],
            CuboidPoint::FrontTopLeftCorner => [0.0, 1.0, 1.0],
            CuboidPoint::FrontTopRightCorner => [0.0, 1.0, 0.0],
            CuboidPoint::BackBottomLeftCorner => [1.0, 0.0, 1.0],
            CuboidPoint::BackBottomRightCorner => [1.0, 0.0, 0.0],
            CuboidPoint::BackTopLeftCorner => [1.0, 1.0, 1.0],
            CuboidPoint::BackTopRightCorner => [1.0, 1.0, 0.0],
            CuboidPoint::FrontTopEdgeCenter => [0.0, 1.0, 0.5],
            CuboidPoint::FrontBottomEdgeCenter => [0.0, 0.0, 0.5],
            CuboidPoint::FrontLeftEdgeCenter => [0.0, 0.5, 1.0],
            CuboidPoint::FrontRightEdgeCenter => [0.0, 0.5, 0.0],
            CuboidPoint::BackTopEdgeCenter => [1.0, 1.0, 0.5],
            CuboidPoint::BackBottomEdgeCenter => [1.0, 0.0, 0.5],
            CuboidPoint::BackLeftEdgeCenter => [1.0, 0.5, 0.0],
            CuboidPoint::BackRightEdgeCenter => [1.0, 0.5, 1.0],
            CuboidPoint::BottomLeftEdgeCenter => [0.5, 0.0, 1.0],
            CuboidPoint::BottomRightEdgeCenter => [0.5, 0.0, 0.0],
            CuboidPoint::TopLeftEdgeCenter => [0.5, 1.0, 1.0],
            CuboidPoint::TopRightEdgeCenter => [0.5, 1.0, 0.0],
        }
    }

    pub fn resolve_in_bounds(&self, min: Vec3, max: Vec3) -> Vec3 {
        let [x, y, z] = self.value();
        let (low_x, high_x) = if min.x < max.x { (min.x, max.x) } else { (max.x, min.x) };
        let (low_y, high_y) = if min.y < max.y { (min.y, max.y) } else { (max.y, min.y) };
        let (low_z, high_z) = if min.z < max.z { (min.z, max.z) } else { (max.z, max.z) };
        Vec3::new(
            low_x + (high_x - low_x) * x,
            low_y + (high_y - low_y) * y,
            low_z + (high_z - low_z) * z,
        )
    }

    pub fn plane_centers_for_bounds(&self, min: Vec3, max: Vec3) -> Vec<Vec3> {
        PLANE_CENTERS.iter().map(|point| {
            point.resolve_in_bounds(min, max)
        }).collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct GrackleCuboid {
    min: Vec3,
    max: Vec3,
}

impl GrackleCuboid {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    
    pub fn get_point(&self, point: CuboidPoint) -> Result<Vec3, PointResolutionError> {
        Ok(point.resolve_in_bounds(self.min, self.max))
    }
}

#[typetag::serde(name = "cuboid")]
impl EditorObject for GrackleCuboid {
    fn get_point(&self, point: &str) -> Result<Vec3, PointResolutionError> {
        todo!()
        //Ok(point.resolve_in_bounds(self.min, self.max))
    }

    fn editor_ui(&mut self, ctx: &mut Context) {
        egui::Window::new(self.type_name()).show(ctx, |ui| {

        });
    }

    fn type_name(&self) -> String {
        get!("editor.actions.cuboid.title")
    }
    
    fn debug_gizmos(&self, _gizmos: &mut Gizmos) {
        todo!()
    }
}
