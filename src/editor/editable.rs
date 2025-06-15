use bevy::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAP_EXT: String = "gmp".to_owned(); // Grackle MaP
    static ref MAP_ART: String = "gma".to_owned(); // Grackle Map Artifact
}

pub enum EditorObjectPoint {
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

impl EditorObjectPoint {
    pub fn value(&self) -> [f64; 3] { // Right-hand y-up: thumb x, index y, middle z
        match self {
            EditorObjectPoint::Centroid => [0.5, 0.5, 0.5],
            EditorObjectPoint::TopPlaneCenter => [0.5, 1.0, 0.5],
            EditorObjectPoint::FrontPlaneCenter => [0.0, 0.5, 0.5],
            EditorObjectPoint::BottomPlaneCenter => [0.5, 0.0, 0.5],
            EditorObjectPoint::BackPlaneCenter => [0.0, 0.5, 0.5],
            EditorObjectPoint::LeftPlaneCenter => [0.5, 0.5, 1.0],
            EditorObjectPoint::RightPlaneCenter => [0.5, 0.5, 0.0],
            EditorObjectPoint::FrontBottomLeftCorner => [0.0, 0.0, 1.0],
            EditorObjectPoint::FrontBottomRightCorner => [0.0, 0.0, 0.0],
            EditorObjectPoint::FrontTopLeftCorner => [0.0, 1.0, 1.0],
            EditorObjectPoint::FrontTopRightCorner => [0.0, 1.0, 0.0],
            EditorObjectPoint::BackBottomLeftCorner => [1.0, 0.0, 1.0],
            EditorObjectPoint::BackBottomRightCorner => [1.0, 0.0, 0.0],
            EditorObjectPoint::BackTopLeftCorner => [1.0, 1.0, 1.0],
            EditorObjectPoint::BackTopRightCorner => [1.0, 1.0, 0.0],
            EditorObjectPoint::FrontTopEdgeCenter => [0.0, 1.0, 0.5],
            EditorObjectPoint::FrontBottomEdgeCenter => [0.0, 0.0, 0.5],
            EditorObjectPoint::FrontLeftEdgeCenter => [0.0, 0.5, 1.0],
            EditorObjectPoint::FrontRightEdgeCenter => [0.0, 0.5, 0.0],
            EditorObjectPoint::BackTopEdgeCenter => [1.0, 1.0, 0.5],
            EditorObjectPoint::BackBottomEdgeCenter => [1.0, 0.0, 0.5],
            EditorObjectPoint::BackLeftEdgeCenter => [1.0, 0.5, 0.0],
            EditorObjectPoint::BackRightEdgeCenter => [1.0, 0.5, 1.0],
            EditorObjectPoint::BottomLeftEdgeCenter => [0.5, 0.0, 1.0],
            EditorObjectPoint::BottomRightEdgeCenter => [0.5, 0.0, 0.0],
            EditorObjectPoint::TopLeftEdgeCenter => [0.5, 1.0, 1.0],
            EditorObjectPoint::TopRightEdgeCenter => [0.5, 1.0, 0.0],
        }
    }
}

pub enum PointResolutionError {
    NoSuchPoint,
    PropagatedError,
    Other,
}

pub trait EditorObject {
    fn get_point(&self, key: &str) -> Result<Vec3, PointResolutionError>;
}

pub struct EditorAction {
    
}
