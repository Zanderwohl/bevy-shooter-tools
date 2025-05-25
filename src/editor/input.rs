use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use std::fmt::Display;
use std::panic::Location;
use bevy::picking::pointer::{PointerId, PointerLocation};
use bevy::tasks::futures_lite::StreamExt;
use crate::editor::multicam::Multicam;

pub struct EditorInputPlugin;

impl Plugin for EditorInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentInput>()
            .add_systems(PreUpdate, Self::mouse_input)
        ;
    }
}

#[derive(Resource)]
pub struct CurrentInput {
    pressed: Option<MouseButton>,
    released: Option<MouseButton>,
    in_camera: Option<Entity>,
    local_pos: Option<Vec2>,
    normalized_pos: Option<Vec2>,
    global_pos: Option<Vec2>,
    world_pos: Option<Ray3d>,
}

impl Default for CurrentInput {
    fn default() -> Self {
        Self {
            pressed: None,
            released: None,
            in_camera: None,
            local_pos: None,
            normalized_pos: None,
            global_pos: None,
            world_pos: None,
        }
    }
}

impl Display for CurrentInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "local: {:?}, normalized: {:?}, global: {:?}", self.local_pos, self.normalized_pos, self.global_pos)
    }
}

impl EditorInputPlugin {
    fn mouse_input(
        mut egui_contexts: EguiContexts,
        primary_window_entity: Query<Entity, With<PrimaryWindow>>,
        primary_window: Query<&Window, With<PrimaryWindow>>,
        mouse_buttons: Res<ButtonInput<MouseButton>>,
        mut current_input: ResMut<CurrentInput>,
        cameras: Query<(Entity, &Camera, &GlobalTransform, &Multicam)>,
        pointers: Query<(&PointerId, &PointerLocation)>,
    ) {
        // We don't want to grab mouse input while over egui windows or panels.
        let ctx = egui_contexts.ctx_mut();
        if ctx.is_pointer_over_area() || ctx.wants_pointer_input() {
            return;
        }

        let window = primary_window.single().unwrap();
        let (pressed, released) = mouse_precedence(mouse_buttons);
        current_input.pressed = pressed;
        current_input.released = released;
        
        // info!("{}", *current_input);
        let mut locations = Vec::new();
        for (_, pointer) in pointers {
            for (camera_entity, camera, camera_transform, cam_multicam) in &cameras {
                if let Some(pointer_loc) = pointer.location() {
                    if pointer_loc.is_in_viewport(camera, &primary_window_entity) {
                        let (position, normalized, ray) = match &camera.viewport {
                            Some(viewport) => {
                                let pos = pointer_loc.position - viewport.physical_position.as_vec2();
                                let normalized = pos / viewport.physical_size.as_vec2();
                                let ray = make_ray(&primary_window_entity, camera, camera_transform, &pointer);
                                (pos, normalized, Some(ray))
                            },
                            None => {
                                let normalized = pointer_loc.position / window.physical_size().as_vec2();
                                (pointer_loc.position, normalized, None)
                            }
                        };
                        locations.push((camera_entity, position, normalized, pointer_loc.position, camera.viewport_to_world(camera_transform, pointer_loc.position).ok(), ray));
                    }
                }
            }
        }
        if let Some(location) = locations.first() {
            current_input.in_camera = Some(location.0);
            current_input.local_pos = Some(location.1);
            current_input.normalized_pos = Some(location.2);
            current_input.global_pos = Some(location.3);
            current_input.world_pos = location.4;
        } else {
            current_input.in_camera = None;
            current_input.local_pos = None;
            current_input.normalized_pos = None;
            current_input.global_pos = None;
            current_input.world_pos = None;
        }
        info!("{}", *current_input);
    }
}

fn make_ray(
    primary_window_entity: &Query<Entity, With<PrimaryWindow>>,
    camera: &Camera,
    camera_tfm: &GlobalTransform,
    pointer_loc: &PointerLocation,
) -> Option<Ray3d> {
    let pointer_loc = pointer_loc.location()?;
    if !pointer_loc.is_in_viewport(camera, primary_window_entity) {
        return None;
    }
    camera.viewport_to_world(camera_tfm, pointer_loc.position).ok()
}

fn mouse_precedence(mouse_buttons: Res<ButtonInput<MouseButton>>) -> (Option<MouseButton>, Option<MouseButton>) {
    let left = mouse_buttons.pressed(MouseButton::Left);
    let right = mouse_buttons.pressed(MouseButton::Right);
    let middle = mouse_buttons.pressed(MouseButton::Middle);
    
    let left_released = mouse_buttons.just_pressed(MouseButton::Left);
    let right_released = mouse_buttons.just_pressed(MouseButton::Right);
    let middle_released = mouse_buttons.just_pressed(MouseButton::Middle);
    
    if !left && !right && !middle {
        if left_released && !right_released && !middle_released {
            return (None, Some(MouseButton::Left));
        }
        if right_released && !left_released && !middle_released {
            return (None, Some(MouseButton::Right));
        }
        if middle_released && !left_released && !right_released {
            return (None, Some(MouseButton::Middle));
        }
    }

    if left && !right && !middle {
        return (Some(MouseButton::Left), None);
    }
    if right && !left && !middle {
        return (Some(MouseButton::Right), None);
    }
    if middle && !left && !right {
        return (Some(MouseButton::Middle), None);
    }

    (None, None)
}
