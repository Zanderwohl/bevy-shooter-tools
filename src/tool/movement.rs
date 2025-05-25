use bevy::app::App;
use bevy::prelude::*;
use crate::editor::input::CurrentInput;
use crate::editor::multicam::Multicam;
use crate::tool::Tools;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                Self::handle,
                )
            )
        ;
    }
}

impl MovementPlugin {
    fn handle(
        current_input: Res<CurrentInput>,
        mut cameras: Query<(Entity, &mut Transform, &Multicam), With<Camera>>,
    ) {
        // For now, let's make middle click orbit for 3d cams and turn for 2d cam
        // and shift + middle click as pan
        if let Some(cam_id) = current_input.in_camera {
            if let Some(button) = current_input.pressed {
                
                if button == MouseButton::Middle {
                    for (entity, mut transform, multicam) in &mut cameras {
                        if cam_id == entity {
                            info!("moving cam {}", multicam.name);
                        }
                    }
                }
            }
        }
    }
}
