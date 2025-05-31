use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use crate::{get, get_with_debug};
use crate::editor::input::{CurrentKeyboardInput, CurrentMouseInput};
use crate::editor::multicam::{CameraAxis, Multicam};
use crate::tool::Tools;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RoomTool>()
            .add_event::<CalculateRoomGeometry>()
            .add_event::<CreateRoom>()
            .add_systems(EguiContextPass, (
                RoomTool::debug_window,
                RoomTool::confirm_window,
            ).run_if(in_state(Tools::Room)))
            .add_systems(Update, (
                RoomTool::interface,
                RoomTool::draw_active,
                RoomTool::draw_handles,
                RoomTool::draw_room_bounds,
                RoomTool::handle_dragging,
                RoomTool::create_active_room,
                ).run_if(in_state(Tools::Room)))
            .add_systems(OnExit(Tools::Room), RoomTool::despawn_handles)
        ;
    }
}

#[derive(Resource)]
struct RoomTool {
    debug_window: bool,
    debug_show_points: bool,
    debug_show_cursor: bool,
    last_min: Vec3,
    last_max: Vec3,
    active_min: Option<Vec3>,
    active_max: Option<Vec3>,
    handles_active: bool,
    handle_mesh: Option<Handle<Mesh>>,
    handle_idle_color: Option<Handle<StandardMaterial>>,
    handle_highlight_color: Option<Handle<StandardMaterial>>,
}

impl Default for RoomTool {
    fn default() -> Self {
        Self {
            debug_window: true,
            debug_show_points: true,
            debug_show_cursor: true,
            last_min: Vec3::ZERO,
            last_max: Vec3::new(10., 10., 10.),
            active_min: None,
            active_max: None,
            handles_active: false,
            handle_mesh: None,
            handle_idle_color: None,
            handle_highlight_color: None,
        }
    }
}

impl RoomTool {
    fn clear(&mut self) {
        self.active_min = None;
        self.active_max = None;
    }
    
    fn create_active_room(
        mut tool: ResMut<Self>,
        mut commands: Commands,
        keyboard_input: Res<CurrentKeyboardInput>,
        mut create_events: EventReader<CreateRoom>,
    ) {
        if !create_events.is_empty() || keyboard_input.confirm {
            create_events.clear();
            let new_room = tool.create();
            if let Some(new_room) = new_room {
                commands.spawn((
                    new_room,
                ));
            }
        }
    }
    
    fn create(&mut self) -> Option<Room> {
        let room = match (self.active_min, self.active_max) {
            (Some(min), Some(max)) => Some(Room::new(min, max)),
            _ => None,
        };
        if let Some(active_min) = self.active_min {
            self.last_min = active_min;
        }
        if let Some(active_max) = self.active_max {
            self.last_max = active_max;
        }
        self.clear();
        room
    }
    
    fn set_min(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        let min = Vec3::new(
            x.unwrap_or(self.last_min.x),
            y.unwrap_or(self.last_min.y),
            z.unwrap_or(self.last_min.z),
        );
        self.active_min = Some(min);
    }
    
    fn set_max(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        if self.active_min.is_none() {
            self.active_min = Some(self.last_min);
        }
        
        let max = Vec3::new(
            x.unwrap_or(self.last_max.x),
            y.unwrap_or(self.last_max.y),
            z.unwrap_or(self.last_max.z),
        );
        self.active_max = Some(max);
    }
    
    fn interface(
        mut tool: ResMut<Self>,
        mut gizmos: Gizmos,
        cameras: Query<(Entity, &Transform, &GlobalTransform, &Multicam, &Projection, &Camera)>,
        mouse_input: Res<CurrentMouseInput>,
    ) {
        if tool.debug_show_points {
            let last_color = Color::srgb_u8(0, 255, 0);
            let active_color = Color::srgb_u8(0, 255, 255);
            gizmos.sphere(tool.last_min, 0.1, last_color);
            gizmos.sphere(tool.last_max, 0.1, last_color);
            if let Some(active_min) = tool.active_min {
                gizmos.sphere(active_min, 0.1, active_color);
            }
            if let Some(active_max) = tool.active_max {
                gizmos.sphere(active_max, 0.1, active_color);
            }
        }
        
        let suggestion = if tool.active_min.is_some() {
            tool.last_max
        } else {
            tool.last_min
        };
        if let Some(camera_entity) = mouse_input.in_camera {
            if let Some(world_pos) = mouse_input.world_pos {
                for (entity, tfm, g_tfm, multicam, _, cam) in cameras {
                    if camera_entity == entity && multicam.axis != CameraAxis::None {
                        let world_pos = world_pos.origin;
                        if tool.debug_show_cursor {
                            let color = Color::srgb_u8(0, 0, 255);
                            gizmos.sphere(world_pos, 0.1, color);
                        }
                        
                        let cursor = match multicam.axis {
                            CameraAxis::None => panic!("{}", get!("debug.room.invalid_cursor")),
                            CameraAxis::X => Vec3::new(suggestion.x, world_pos.y, world_pos.z),
                            CameraAxis::Y => Vec3::new(world_pos.x, suggestion.y, world_pos.z),
                            CameraAxis::Z => Vec3::new(world_pos.x, world_pos.y, suggestion.z),
                        };
                        let color = Color::srgb_u8(255, 0, 0);
                        gizmos.sphere(cursor, 0.2, color);
                        
                        if let Some(button) = mouse_input.released {
                            if button == MouseButton::Left {
                                if tool.active_min.is_none() {
                                    tool.active_min = Some(cursor);
                                } else if tool.active_max.is_none() {
                                    tool.active_max = Some(cursor);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn draw_room_bounds(
        mut gizmos: Gizmos,
        rooms: Query<(Entity, &Room)>,
    ) {
        let color = Color::srgb_u8(100, 100, 100);
        for (_, room) in rooms {
            Self::bounds_gizmo(&mut gizmos, room.min, room.max, color);
        }
    }

    fn draw_active(
        tool: Res<RoomTool>,
        mut gizmos: Gizmos,
    ) {
        match (tool.active_min, tool.active_max) {
            (Some(min), Some(max)) => {
                let color = Color::srgb_u8(200, 200, 200);
                Self::bounds_gizmo(&mut gizmos, min, max, color);
            }
            _ => {}
        }
    }

    fn bounds_gizmo(gizmos: &mut Gizmos, min: Vec3, max: Vec3, color: Color) {
        // Bottom face (z = min.z)
        gizmos.line(Vec3::new(min.x, min.y, min.z), Vec3::new(max.x, min.y, min.z), color);
        gizmos.line(Vec3::new(max.x, min.y, min.z), Vec3::new(max.x, max.y, min.z), color);
        gizmos.line(Vec3::new(max.x, max.y, min.z), Vec3::new(min.x, max.y, min.z), color);
        gizmos.line(Vec3::new(min.x, max.y, min.z), Vec3::new(min.x, min.y, min.z), color);

        // Top face (z = max.z)
        gizmos.line(Vec3::new(min.x, min.y, max.z), Vec3::new(max.x, min.y, max.z), color);
        gizmos.line(Vec3::new(max.x, min.y, max.z), Vec3::new(max.x, max.y, max.z), color);
        gizmos.line(Vec3::new(max.x, max.y, max.z), Vec3::new(min.x, max.y, max.z), color);
        gizmos.line(Vec3::new(min.x, max.y, max.z), Vec3::new(min.x, min.y, max.z), color);

        // Vertical edges connecting bottom and top faces
        gizmos.line(Vec3::new(min.x, min.y, min.z), Vec3::new(min.x, min.y, max.z), color);
        gizmos.line(Vec3::new(max.x, min.y, min.z), Vec3::new(max.x, min.y, max.z), color);
        gizmos.line(Vec3::new(max.x, max.y, min.z), Vec3::new(max.x, max.y, max.z), color);
        gizmos.line(Vec3::new(min.x, max.y, min.z), Vec3::new(min.x, max.y, max.z), color);
    }

    fn centroid(&self) -> Option<Vec3> {
        match (self.active_min, self.active_max) {
            (Some(min), Some(max)) => Some((max + min) / 2.0),
            _ => None,
        }
    }

    fn size(&self) -> Vec3 {
        match (self.active_min, self.active_max) {
            (Some(min), Some(max)) => max - min,
            _ => Vec3::ZERO,
        }
    }

    fn face_centers(&self) -> Vec<Vec3> {
        let mut face_centers: Vec<Vec3> = Vec::new();
        self.centroid().map(|centroid| {
            let half_size = self.size() / 2.0;

            face_centers.push(Vec3::new(centroid.x + half_size.x, centroid.y, centroid.z));
            face_centers.push(Vec3::new(centroid.x - half_size.x, centroid.y, centroid.z));

            face_centers.push(Vec3::new(centroid.x, centroid.y + half_size.y, centroid.z));
            face_centers.push(Vec3::new(centroid.x, centroid.y - half_size.y, centroid.z));

            face_centers.push(Vec3::new(centroid.x, centroid.y, centroid.z + half_size.z));
            face_centers.push(Vec3::new(centroid.x, centroid.y, centroid.z - half_size.z));
        });
        face_centers
    }

    fn draw_handles(
        mut tool: ResMut<Self>,
        handles: Query<Entity, With<RoomToolHandle>>,
        mut gizmos: Gizmos,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let face_centers = tool.face_centers();
        
        if let (Some(min), Some(max)) = (tool.active_min, tool.active_max) {
            // spawn handles
            if !tool.handles_active {
                tool.handles_active = true;
                for center in &face_centers {
                    if tool.handle_mesh.is_none() {
                        let mesh = meshes.add(Cuboid::new(0.3, 0.3, 0.3));
                        //let idle_material = materials.add(Color::srgb_u8(180, 230, 180));
                        let idle_material = materials.add(StandardMaterial {
                            base_color: Color::srgb_u8(180, 230, 180),
                            emissive: LinearRgba::rgb(0.2, 0.3, 0.2),
                            ..Default::default()
                        });
                        let highlight_material = materials.add(Color::srgb_u8(220, 255, 220));
                        tool.handle_mesh = Some(mesh);
                        tool.handle_idle_color = Some(idle_material);
                        tool.handle_highlight_color = Some(highlight_material);
                    }
                    
                    match (&tool.handle_mesh, &tool.handle_idle_color) {
                        (Some(mesh), Some(color)) => {
                            commands.spawn((
                                RoomToolHandle,
                                Mesh3d(mesh.clone()),
                                MeshMaterial3d(color.clone()),
                                Transform::from_translation(*center),
                            ));
                        }
                        _ => panic!("{}", get!("room.missing_material"))
                    }
                    
                }
            }
        }
        
        // Despawn handles as appropriate
        if tool.active_min.is_none() || tool.active_max.is_none() {
            tool.handles_active = false;
            crate::common::systems::despawn_entities_with::<RoomToolHandle>(commands, handles);
        }
    }
    
    fn handle_dragging(
        handles: Query<Entity, With<RoomToolHandle>>,
        window: Query<&Window, With<PrimaryWindow>>,
        mut ray_cast: MeshRayCast,
        mouse_input: Res<CurrentMouseInput>,
    ) {
        let window = window.single();
        if window.is_err() {
            return;
        }
        let window = window.unwrap();
        
        let filter = |entity| handles.get(entity).is_ok();
        let settings = MeshRayCastSettings::default().with_filter(&filter);
        
        if let Some(ray) = mouse_input.world_pos {
            if let Some((hit_entity, hit_data)) = ray_cast
                .cast_ray(ray, &settings)
                .first() {
                info!("{}", hit_entity);
            }
        }
    }
    
    fn confirm_window(
        mut tool: ResMut<Self>,
        mut contexts: EguiContexts,
        mut create_room: EventWriter<CreateRoom>,
    ) {
        let ctx = contexts.try_ctx_mut();
        if ctx.is_none() { return; }
        let ctx = ctx.unwrap();

        if let (Some(min), Some(max)) = (tool.active_min, tool.active_max) {
            egui::Window::new(get!("room.confirm.title")).show(ctx, |ui| {
                if ui.button(get!("room.confirm.confirm")).clicked() {
                    create_room.write(CreateRoom);
                }
            });
        }
    }

    fn despawn_handles(handles: Query<Entity, With<RoomToolHandle>>, mut commands: Commands, mut tool: ResMut<Self>) {
        crate::common::systems::despawn_entities_with::<RoomToolHandle>(commands, handles);
        tool.handles_active = false;
    }
    
    fn debug_window(
        mut contexts: EguiContexts,
        mut tool: ResMut<Self>,
    ) {
        let ctx = contexts.try_ctx_mut();
        if ctx.is_none() { return; }
        let ctx = ctx.unwrap();
        
        if !tool.debug_window {
            return;
        }
        
        let active_min = tool.active_min.map(|m| format!("{}", m)).unwrap_or("None".to_owned());
        let active_max = tool.active_max.map(|m| format!("{}", m)).unwrap_or("None".to_owned());
        
        egui::Window::new(get!("debug.room.title")).show(ctx, |ui| {
            ui.heading(get!("debug.room.state"));
            ui.label(get!("debug.room.last_min", "x", tool.last_min));
            ui.label(get!("debug.room.last_max", "x", tool.last_max));
            ui.label(get!("debug.room.active_min", "x", active_min));
            ui.label(get!("debug.room.active_max", "x", active_max));
            
            ui.checkbox(&mut tool.debug_show_points, get!("debug.room.show_points"));
            ui.checkbox(&mut tool.debug_show_cursor, get!("debug.room.show_cursor"));
        });
    }
}

#[derive(Component)]
pub struct RoomToolHandle;

#[derive(Component)]
pub struct Room {
    min: Vec3,
    max: Vec3,
    ghost: Option<Entity>,  // This is set if this room is completely inside another room.
}

impl Default for Room {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Vec3::ONE)
    }
}

impl Room {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min,
            max,
            ghost: None,
        }
    }
    
    pub fn messages(&self, my_entity: Entity) -> Vec<String> {
        let mut messages = Vec::new();
        if let Some(entity) = self.ghost {
            messages.push(get!("room.messages.ghost", "me", my_entity, "other", entity));
        }
        messages
    }
    
    pub fn point_inside(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
        && point.y >= self.min.y && point.y <= self.max.y
        && point.z >= self.min.z && point.z <= self.max.z
    }
    
    pub fn extremes(&self) -> Vec<Vec3> {
        let mut extremes = Vec::with_capacity(8);
        
        extremes.push(Vec3::new(self.min.x, self.min.y, self.min.z));
        extremes.push(Vec3::new(self.max.x, self.min.y, self.min.z));
        extremes.push(Vec3::new(self.max.x, self.max.y, self.min.z));
        extremes.push(Vec3::new(self.min.x, self.max.y, self.min.z));

        extremes.push(Vec3::new(self.min.x, self.min.y, self.max.z));
        extremes.push(Vec3::new(self.max.x, self.min.y, self.max.z));
        extremes.push(Vec3::new(self.max.x, self.max.y, self.max.z));
        extremes.push(Vec3::new(self.min.x, self.max.y, self.max.z));
        
        extremes
    }
    pub fn count_points_inside(&self, points: &Vec<Vec3>) -> usize {
        points.iter().map(|p| self.point_inside(p.clone()) as usize).sum() 
    }
    
    pub fn test_intersection(left: &Self, right: &Self) -> IntersectionResult {
        let engulfed_right_points = left.count_points_inside(&right.extremes());
        let engulfed_left_points = right.count_points_inside(&left.extremes());
        if engulfed_right_points == 0 || engulfed_left_points == 0 {
            return IntersectionResult::None
        }
        if engulfed_right_points == 8 && engulfed_left_points == 8 {
            return IntersectionResult::Identical
        }
        if engulfed_right_points == 8 {
            return IntersectionResult::LeftEngulfsRight
        }
        if engulfed_left_points == 8 {
            return IntersectionResult::RightEngulfsLeft
        }
        IntersectionResult::Intersection
    }
}

pub enum IntersectionResult {
    None,
    LeftEngulfsRight,
    RightEngulfsLeft,
    Identical,
    Intersection,
}

#[derive(Event)]
pub struct CalculateRoomGeometry;

#[derive(Event)]
pub struct CreateRoom;

#[cfg(test)]
mod tests {
    use bevy::ecs::relationship::RelationshipSourceCollection;
    use bevy::prelude::*;
    use super::*;
    
    #[test]
    fn test_no_messages() {
        let a = Entity::from_raw(23);
        let b = Entity::from_raw(45);

        let good_room = Room::default();
        let no_messages = good_room.messages(a);
        assert_eq!(no_messages.len(), 0);
    }

    #[test]
    fn test_ghost_message() {
        let a = Entity::from_raw(23);
        let b = Entity::from_raw(45);
        
        let mut ghost_room = Room::default();
        ghost_room.ghost = Some(b);
        let ghost_message = ghost_room.messages(a);
        assert_eq!(ghost_message.len(), 1);
        assert_eq!(ghost_message[0], "Room 23v1 is fully inside 45v1 and will not appear!");
    }
    
    #[test]
    fn test_point_inside() {
        let room = Room::new(Vec3::ZERO, Vec3::ONE);
        
        assert!(room.point_inside(Vec3::ZERO));
        assert!(room.point_inside(Vec3::ONE));
        assert!(room.point_inside(Vec3::new(0.5, 0.5, 0.5)));
        assert!(room.point_inside(Vec3::new(0.5, 1.0, 0.5)));
        
        assert!(!room.point_inside(Vec3::new(0.5, 1.1, 0.5)));
        assert!(!room.point_inside(Vec3::new(1.1, 0.5, 0.5)));
        assert!(!room.point_inside(Vec3::new(0.5, 0.5, 1.1)));
        assert!(!room.point_inside(Vec3::new(0.5, -1.1, 0.5)));
        assert!(!room.point_inside(Vec3::new(-1.1, 0.5, 0.5)));
        assert!(!room.point_inside(Vec3::new(0.5, 0.5, -1.1)));
    }
}
