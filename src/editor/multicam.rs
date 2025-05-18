use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;

pub struct MulticamPlugin {
    pub test_scene: bool,
}

#[derive(Resource)]
pub struct MulticamState {
    pub test_scene: bool,
}

#[derive(Component)]
pub struct Multicam {
    pub name: String,
    screen_pos: UVec2,
}

#[derive(Component)]
pub struct MulticamTestScene;

impl Default for MulticamState {
    fn default() -> Self {
        Self {
            test_scene: false,
        }
    }
}

impl Plugin for MulticamPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MulticamState {
                test_scene: self.test_scene,
            })
            .add_systems(Startup, Self::setup)
            .add_systems(Update, Self::set_camera_viewports)
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
        let cameras = [
            ("Free Camera", Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y), true),
            ("X Camera", Transform::from_xyz(5.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y), false),
            ("Y Camera", Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, -Vec3::X), false),
            ("Z Camera", Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), false),
        ];
        for (idx, (camera_name, camera_pos, perspective)) in cameras.into_iter().enumerate() {
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
                        screen_pos: UVec2::new((idx % 2) as u32, (idx / 2) as u32)
                    },
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
        windows: Query<&Window>,
        mut resize_events: EventReader<WindowResized>,
        mut cameras: Query<(&mut Camera, &Multicam)>,
    ) {
        for resize_event in resize_events.read() {
            let window = windows.get(resize_event.window).unwrap();
            let size = window.physical_size() / 2;

            for (mut camera, multicam) in &mut cameras {
                camera.viewport = Some(Viewport {
                    physical_position: multicam.screen_pos * size,
                    physical_size: size,
                    ..Default::default()
                });
            }
        }
    }
}
