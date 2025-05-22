use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::FrameCount;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::picking::backend::ray::RayMap;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use bevy_vector_shapes::prelude::*;
use crate::get;

pub struct MulticamPlugin {
    pub test_scene: bool,
}

#[derive(Resource)]
pub struct MulticamState {
    pub test_scene: bool,
    pub start: Vec2,
    pub end: Vec2,
    pub debug_window: bool,
    pub debug_viewport_box: bool,
    pub debug_mouse_circle: bool,
}

#[derive(Component)]
pub struct Multicam {
    pub name: String,
    pub screen_pos: UVec2,
    pub id: u32,
}

#[derive(Component)]
pub struct MulticamTestScene;

#[derive(Component)]
pub struct EditorSelectable {
    id: String,
}

impl Default for MulticamState {
    fn default() -> Self {
        Self {
            test_scene: false,
            start: Vec2::new(0.1, 0.1),
            end: Vec2::new(0.9, 0.9), // This MUST be more than start or else the first frame will crash.
            debug_viewport_box: false,
            debug_mouse_circle: false,
            debug_window: true,
        }
    }
}

impl Plugin for MulticamPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MulticamState {
                test_scene: self.test_scene,
                ..Default::default()
            })
            .add_systems(Startup, Self::setup)
            .add_systems(Update, (
                Self::set_camera_viewports,
                Self::handle_input,
            ))
            .add_systems(EguiContextPass, Self::debug_window)
        ;
    }
}

impl MulticamPlugin {
    fn setup(
        mut commands: Commands,
        state: Res<MulticamState>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let perspective = Projection::Perspective(PerspectiveProjection {
            fov: 120.0,
            ..Default::default()
        });
        let orthographic = Projection::Orthographic(OrthographicProjection {
            near: 0.05,
            far: 1000.0,
            scaling_mode: Default::default(),
            scale: 0.01,
            ..OrthographicProjection::default_2d()
        });

        let cameras = [
            (get!("viewport.free"), Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y), &perspective),
            (get!("viewport.front"), Transform::from_xyz(5.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y), &orthographic),
            (get!("viewport.top"), Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, -Vec3::X), &orthographic),
            (get!("viewport.right"), Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), &orthographic),
        ];
        let cameras_len = cameras.len();

        commands.spawn((
            Camera2d::default(),
            GlobalTransform::default(),
            Camera {
                hdr: true,
                order: (cameras_len + 1) as isize,
                ..Default::default()
            },
            /*Multicam {
                name: get!("viewport.ui"),
                screen_pos: UVec2::new(0u32, 0u32),
                id: cameras_len as u32 + 1,
            },*/
        ));

        for (idx, (camera_name, camera_pos, projection)) in cameras.into_iter().enumerate() {
            let camera = commands
                .spawn((
                    Camera3d::default(),
                    Camera {
                        hdr: true,
                        order: (cameras_len - idx) as isize,
                        ..Default::default()
                    },
                    camera_pos,
                    Bloom::NATURAL,
                    Tonemapping::TonyMcMapface,
                    Multicam {
                        name: camera_name.to_string(),
                        screen_pos: UVec2::new((idx % 2) as u32, (idx / 2) as u32),
                        id: idx as u32,
                    },
                    projection.clone(),
                ))
                .id();

                commands
                    .spawn((
                        UiTargetCamera(camera),
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..Default::default()
                        }
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Px(12.),
                                left: Val::Px(12.),
                                ..Default::default()
                            },
                            Text::new(camera_name),
                        ));
                    });
        }

        // Only spawn the test cube if test_scene is true
        if state.test_scene {
            // circular base
            commands.spawn((
                Mesh3d(meshes.add(Circle::new(4.0))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                MulticamTestScene,
                EditorSelectable { id: "Base".to_owned() },
            ));
            // cube
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                Transform::from_xyz(0.0, 0.5, 0.0),
                MulticamTestScene,
                EditorSelectable { id: "Cube".to_owned() },
            ));
            // light
            commands.spawn((
                PointLight {
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(4.0, 8.0, 4.0),
                MulticamTestScene,
            ));
        }
    }

    fn set_camera_viewports(
        windows: Query<&Window, With<PrimaryWindow>>,
        mut resize_events: EventReader<WindowResized>,
        mut cameras: Query<(&mut Camera, &Multicam)>,
        state: Res<MulticamState>,
        frames: Res<FrameCount>,
    ) {
        for resize_event in resize_events.read() {
            let window = windows.get(resize_event.window).unwrap();
            Self::calculate_resize(&mut cameras, &state, window);
        }
        if state.is_changed() {
            let window = windows.single().unwrap();
            Self::calculate_resize(&mut cameras, &state, window);
        }
        if frames.0 < 3 {
            let window = windows.single().unwrap();
            Self::calculate_resize(&mut cameras, &state, window);
        }
    }
    
    fn calculate_resize(cameras: &mut Query<(&mut Camera, &Multicam)>, state: &Res<MulticamState>, window: &Window) {
        let window_size = window.physical_size();

        // Calculate the viewport size based on start and end coordinates
        let viewport_size = UVec2::new(
            ((state.end.x - state.start.x) * window_size.x as f32) as u32,
            ((state.end.y - state.start.y) * window_size.y as f32) as u32,
        );

        // Calculate the starting position of the viewport
        let viewport_start = UVec2::new(
            (state.start.x * window_size.x as f32) as u32,
            (state.start.y * window_size.y as f32) as u32,
        );

        // Calculate the size of each camera's viewport (2x2 grid)
        let camera_size = UVec2::new(
            viewport_size.x / 2,
            viewport_size.y / 2,
        );

        for (mut camera, multicam) in cameras {
            // Calculate this camera's position within the viewport
            let camera_pos = viewport_start + UVec2::new(
                multicam.screen_pos.x * camera_size.x,
                multicam.screen_pos.y * camera_size.y,
            );

            camera.viewport = Some(Viewport {
                physical_position: camera_pos,
                physical_size: camera_size,
                ..Default::default()
            });
        }
    }

    fn debug_window(
        mut state: ResMut<MulticamState>,
        mut contexts: EguiContexts,
    ) {
        let ctx = contexts.ctx_mut();
        
        if !state.debug_window {
            return;
        }

        egui::Window::new(get!("debug.viewport.title")).show(ctx, |ui| {
            ui.heading(get!("debug.viewport.controls"));
            
            // Start coordinates
            ui.heading("X");
            ui.horizontal(|ui| {
                ui.label(get!("debug.viewport.start"));
                let mut start_x = state.start.x;
                if ui.add(egui::Slider::new(&mut start_x, 0.0..=state.end.x - 0.01)).changed() {
                    state.start.x = start_x;
                }
            });
            ui.horizontal(|ui| {
                ui.label(get!("debug.viewport.end"));
                let mut end_x = state.end.x;
                if ui.add(egui::Slider::new(&mut end_x, (state.start.x + 0.01)..=1.0)).changed() {
                    state.end.x = end_x;
                }
            });

            ui.separator();

            // End coordinates
            ui.heading("Y");
            ui.horizontal(|ui| {
                ui.label(get!("debug.viewport.start"));
                let mut start_y = state.start.y;
                if ui.add(egui::Slider::new(&mut start_y, 0.0..=state.end.y - 0.01)).changed() {
                    state.start.y = start_y;
                }
            });
            ui.horizontal(|ui| {
                ui.label(get!("debug.viewport.end"));
                let mut end_y = state.end.y;
                if ui.add(egui::Slider::new(&mut end_y, (state.start.y + 0.01)..=1.0)).changed() {
                    state.end.y = end_y;
                }
            });
            
            ui.separator();
            ui.heading(get!("debug.viewport.draw.title"));
            ui.checkbox(&mut state.debug_mouse_circle, get!("debug.viewport.draw.mouse"));
            ui.checkbox(&mut state.debug_viewport_box, get!("debug.viewport.draw.box"));
        });
    }

    fn handle_input(
        state: ResMut<MulticamState>,
        mouse_buttons: Res<ButtonInput<MouseButton>>,
        windows: Query<&Window, With<PrimaryWindow>>,
        cameras_q: Query<(Entity, &Camera, &GlobalTransform, &Multicam)>,
        ui_cam: Query<(&Camera, &Camera2d), Without<Multicam>>,
        mut painter: ShapePainter,
        mut _evr_motion: EventReader<MouseMotion>,
        mut egui_contexts: EguiContexts,
        ray_map: Res<RayMap>,
        mut ray_cast: MeshRayCast,
        selectables: Query<&EditorSelectable>,
        mut gizmos: Gizmos,
    ) {
        let ctx = egui_contexts.ctx_mut();
        if ctx.is_pointer_over_area() || ctx.wants_pointer_input() {
            return;
        }

        let window = windows.single().unwrap();
        let (ui_cam, _) = ui_cam.single().unwrap();

        let left_pressed = mouse_buttons.pressed(MouseButton::Left);
        let right_pressed = mouse_buttons.pressed(MouseButton::Right);

        let mut button: Option<MouseButton> = None;

        if left_pressed && right_pressed {
            // If both were just pressed, discard for this interaction
        } else if left_pressed {
            button = Some(MouseButton::Left);
        } else if right_pressed {
            button = Some(MouseButton::Right);
        }

        
        if state.debug_viewport_box {
            painter.reset();
            let viewport_start = window_to_painter_frac(&ui_cam, state.start).extend(1.0);
            let viewport_end = window_to_painter_frac(&ui_cam, state.end).extend(1.0);
            painter.color = Color::srgb_u8(0, 0, 255);
            draw_rect(&mut painter, viewport_start.truncate(), viewport_end.truncate());
        }

        if state.debug_mouse_circle {
            if let Some(cursor_window_pos) = window.cursor_position() {
                painter.reset();
                painter.set_translation(window_to_painter(&ui_cam, cursor_window_pos).extend(1.0));
                painter.circle(10.0);
            }
        }

        let filter = |entity| selectables.get(entity).is_ok();
        let settings = MeshRayCastSettings::default().with_filter(&filter);

        if let Some(cursor_pos_window) = window.cursor_position() {
            for (id, camera, camera_transform, camera_multicam) in cameras_q.iter() {
                for (ray_id, ray) in ray_map.iter() {
                    if id == ray_id.camera {
                        if let Some((hit_entity, hit_data)) = ray_cast
                            .cast_ray(*ray, &settings)
                            .first() {
                            if let Ok(selectable) = selectables.get(*hit_entity) {
                                // draw the ray in 3d
                                gizmos.line(ray.origin, hit_data.point, Color::srgb_u8(255, 0, 0));

                                info!("In Camera {} Selectable {:?}", camera_multicam.name, selectable.id);
                            }
                        }
                    }
                }

                
                if let Some(viewport) = &camera.viewport {
                    let vp_min = viewport.physical_position.as_vec2();
                    let vp_max = vp_min + viewport.physical_size.as_vec2();

                    let physical_cursor_x = cursor_pos_window.x * window.scale_factor();
                    let physical_cursor_y = cursor_pos_window.y * window.scale_factor();

                    // Check if cursor is within this viewport's bounds
                    if physical_cursor_x >= vp_min.x && physical_cursor_x < vp_max.x &&
                        physical_cursor_y >= vp_min.y && physical_cursor_y < vp_max.y
                    {
                        if let Some(button) = button {
                            if mouse_buttons.pressed(button) {
                                Self::draw_indicator_box(&mut painter, &ui_cam, viewport, button);
                                
                                
                            }
                        } else {
                            painter.reset(); // Reset painter properties for this specific drawing
                            painter.color = Color::srgb_u8(200, 200, 200);
                            painter.thickness = 1.0; // Define border thickness

                            let min = viewport.physical_position.as_vec2();
                            let max = min + viewport.physical_size.as_vec2();
                            let min = window_to_painter(&ui_cam, min);
                            let max = window_to_painter(&ui_cam, max);

                            draw_rect(&mut painter, min, max);
                        }

                        break; // Border drawn for the first viewport found under cursor
                    }
                }
            }
        }
    }

    fn draw_indicator_box(mut painter: &mut ShapePainter, ui_cam: &&Camera, viewport: &Viewport, button: MouseButton) {
        let color = match button {
            MouseButton::Left => Color::WHITE,
            MouseButton::Right => Color::hsv(0.0, 0.4, 1.0),
            _ => Color::srgb_u8(0, 255, 0), // Should not be reached due to earlier logic
        };

        painter.reset(); // Reset painter properties for this specific drawing
        painter.color = color;
        painter.thickness = 2.0; // Define border thickness

        let min = viewport.physical_position.as_vec2();
        let max = min + viewport.physical_size.as_vec2();
        let min = window_to_painter(&ui_cam, min);
        let max = window_to_painter(&ui_cam, max);

        draw_rect(&mut painter, min, max);
    }
}

fn draw_rect(
    painter: &mut ShapePainter,
    min: Vec2,
    max: Vec2,
) {
    painter.line(Vec3::new(min.x, min.y, 0.0), Vec3::new(max.x, min.y, 0.0)); // Bottom
    painter.line(Vec3::new(min.x, max.y, 0.0), Vec3::new(max.x, max.y, 0.0)); // Top
    painter.line(Vec3::new(min.x, min.y, 0.0), Vec3::new(min.x, max.y, 0.0)); // Left
    painter.line(Vec3::new(max.x, min.y, 0.0), Vec3::new(max.x, max.y, 0.0)); // Right
}

fn window_to_painter(cam: &Camera, pos: Vec2) -> Vec2 {
    let cam_viewport = cam.physical_viewport_rect().unwrap();
    let size_x = (cam_viewport.max.x - cam_viewport.min.x) as f32;
    let size_y = (cam_viewport.max.y - cam_viewport.min.y) as f32;
    Vec2::new(
        (cam_viewport.max.x - cam_viewport.min.x) as f32 * (pos.x / size_x - 0.5),
        (cam_viewport.max.y - cam_viewport.min.y) as f32 * (1.0 - (pos.y / size_y + 0.5))
    )
}

fn window_to_painter_broken(cam: &Camera, pos: Vec2, state: &MulticamState) -> Vec2 {
    let cam_viewport = cam.physical_viewport_rect().unwrap();
    let size_x = (cam_viewport.max.x - cam_viewport.min.x) as f32 * (state.end.x - state.start.x);
    let size_y = (cam_viewport.max.y - cam_viewport.min.y) as f32 * (state.end.y - state.start.y);
    Vec2::new(
        (cam_viewport.max.x - cam_viewport.min.x) as f32 * (pos.x / size_x - 0.5),
        (cam_viewport.max.y - cam_viewport.min.y) as f32 * (1.0 - (pos.y / size_y + 0.5))
    )
}

fn window_to_painter_frac(cam: &Camera, frac: Vec2) -> Vec2 {
    let cam_viewport = cam.physical_viewport_rect().unwrap();
    Vec2::new(
        (cam_viewport.max.x - cam_viewport.min.x) as f32 * (frac.x - 0.5),
        (cam_viewport.max.y - cam_viewport.min.y) as f32 * (1.0 - (frac.y + 0.5))
    )
}
