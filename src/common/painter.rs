use bevy::prelude::Camera;
use bevy::math::Vec2;

pub fn window_to_painter(cam: &Camera, pos: Vec2) -> Vec2 {
    let cam_viewport = cam.physical_viewport_rect().unwrap();
    let size_x = (cam_viewport.max.x - cam_viewport.min.x) as f32;
    let size_y = (cam_viewport.max.y - cam_viewport.min.y) as f32;
    Vec2::new(
        (cam_viewport.max.x - cam_viewport.min.x) as f32 * (pos.x / size_x - 0.5),
        (cam_viewport.max.y - cam_viewport.min.y) as f32 * (1.0 - (pos.y / size_y + 0.5))
    )
}

pub fn window_to_painter_frac(cam: &Camera, frac: Vec2) -> Vec2 {
    let cam_viewport = cam.physical_viewport_rect().unwrap();
    Vec2::new(
        (cam_viewport.max.x - cam_viewport.min.x) as f32 * (frac.x - 0.5),
        (cam_viewport.max.y - cam_viewport.min.y) as f32 * (1.0 - (frac.y + 0.5))
    )
}