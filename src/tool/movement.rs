use bevy::app::App;
use bevy::prelude::*;
use crate::editor::input::{CurrentKeyboardInput, CurrentMouseInput};
use crate::editor::multicam::Multicam;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MovementSettings>()
            .add_systems(Update, (
                Self::handle,
                )
            )
        ;
    }
}

impl MovementPlugin {
    fn handle(
        movement_settings: Res<MovementSettings>,
        mouse_input: Res<CurrentMouseInput>,
        keyboard_input: Res<CurrentKeyboardInput>,
        mut cameras: Query<(Entity, &mut Transform, &Multicam, &Projection, &Camera)>,
    ) {
        // For now, let's make middle click orbit for 3d cams and turn for 2d cam
        // and shift + middle click as pan
        if let Some(cam_id) = mouse_input.started_in_camera {
            if let Some(button) = mouse_input.pressed {
                if button == MouseButton::Middle {
                    for (entity, mut transform, multicam, projection, camera) in &mut cameras {
                        if cam_id == entity {
                            let delta = mouse_input.delta_pos;
                            match projection {
                                Projection::Perspective(projection) => {
                                    if keyboard_input.modify {
                                        let pan_scaled_x = delta.x * movement_settings.perspective_pan;
                                        let pan_scaled_y = delta.y * movement_settings.perspective_pan;

                                        let local_x = transform.local_x();
                                        transform.translation -= local_x * pan_scaled_x;
                                        let local_y = transform.local_y();
                                        transform.translation += local_y * pan_scaled_y;
                                    }
                                    
                                }
                                Projection::Orthographic(projection) => {
                                    let pan_scaled_x = delta.x * projection.scale;
                                    let pan_scaled_y = delta.y * projection.scale;
                                    
                                    let local_x = transform.local_x();
                                    transform.translation -= local_x * pan_scaled_x;
                                    let local_y = transform.local_y();
                                    transform.translation += local_y * pan_scaled_y;
                                }
                                Projection::Custom(_) => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct MovementSettings {
    perspective_pan: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            perspective_pan: 0.1,
        }
    }
}