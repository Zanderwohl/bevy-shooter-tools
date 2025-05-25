use bevy::app::App;
use bevy::prelude::*;
use crate::editor::multicam::Multicam;
use crate::tool::Tools;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementEvent>()
            .add_systems(Update, (
                Self::handle,
                ).run_if(in_state(Tools::Move))
            )
        ;
    }
}

#[derive(Event)]
pub struct MovementEvent {
    pub mouse_button: Option<MouseButton>,
    pub mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub camera: Entity,
    pub camera_translation: GlobalTransform,
}

impl MovementPlugin {
    fn handle(
        mut events: EventReader<MovementEvent>,
        mut cameras: Query<(Entity, &mut Transform, &Multicam), With<Camera>>,
    ) {
        // For now, let's make middle click orbit for 3d cams and turn for 2d cam
        // and shift + middle click as pan
        for event in events.read() {
            let cam_id = event.camera;
            for (entity, mut transform, multicam) in &mut cameras {
                if cam_id == entity {
                    info!("moving cam {}", multicam.name);
                }
            }
        }
    }
}
