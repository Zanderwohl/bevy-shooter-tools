use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use bevy_vector_shapes::prelude::*;

pub struct MulticamPlugin {
    pub test_scene: bool,
}

#[derive(Resource)]
pub struct MulticamState {
    pub test_scene: bool,
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Component)]
pub struct Multicam {
    pub name: String,
    pub screen_pos: UVec2,
    pub id: u32,
}

#[derive(Component)]
pub struct MulticamTestScene;

impl Default for MulticamState {
    fn default() -> Self {
        Self {
            test_scene: false,
            start: Vec2::ZERO,
            end: Vec2::ONE, // This MUST be more than start or else the first frame will crash.
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
            ("Free Camera", Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y), &perspective),
            ("Front", Transform::from_xyz(5.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y), &orthographic),
            ("Top", Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, -Vec3::X), &orthographic),
            ("Right", Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), &orthographic),
        ];
        for (idx, (camera_name, camera_pos, projection)) in cameras.into_iter().enumerate() {
            let camera = commands
                .spawn((
                    Camera3d::default(),
                    Camera {
                        hdr: true,
                        order: (cameras.len() - idx) as isize,
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
            ));
            // cube
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                Transform::from_xyz(0.0, 0.5, 0.0),
                MulticamTestScene,
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
    ) {
        for resize_event in resize_events.read() {
            let window = windows.get(resize_event.window).unwrap();
            Self::calculate_resize(&mut cameras, &state, window);
        }
        if state.is_changed() {
            let window = windows.single().unwrap();
            Self::calculate_resize(&mut cameras, &state, window);
        }
    }

    fn calculate_resize(mut cameras: &mut Query<(&mut Camera, &Multicam)>, state: &Res<MulticamState>, window: &Window) {
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

        egui::Window::new("Multicam Viewport").show(ctx, |ui| {
            ui.heading("Viewport Controls");
            
            // Start coordinates
            ui.heading("Start Position");
            ui.horizontal(|ui| {
                ui.label("X:");
                let mut start_x = state.start.x;
                if ui.add(egui::Slider::new(&mut start_x, 0.0..=state.end.x - 0.01)).changed() {
                    state.start.x = start_x;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Y:");
                let mut start_y = state.start.y;
                if ui.add(egui::Slider::new(&mut start_y, 0.0..=state.end.y - 0.01)).changed() {
                    state.start.y = start_y;
                }
            });

            ui.separator();

            // End coordinates
            ui.heading("End Position");
            ui.horizontal(|ui| {
                ui.label("X:");
                let mut end_x = state.end.x;
                if ui.add(egui::Slider::new(&mut end_x, (state.start.x + 0.01)..=1.0)).changed() {
                    state.end.x = end_x;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Y:");
                let mut end_y = state.end.y;
                if ui.add(egui::Slider::new(&mut end_y, (state.start.y + 0.01)..=1.0)).changed() {
                    state.end.y = end_y;
                }
            });
        });
    }

    fn handle_input(
        mut state: ResMut<MulticamState>,
        mouse_buttons: Res<ButtonInput<MouseButton>>,
        windows: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<(&Camera, &GlobalTransform, &Multicam)>,
        mut painter: ShapePainter,
    ) {
        let window = windows.single().unwrap();

        let mouse = window.cursor_position();
    }
}
