use bevy::app::App;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use crate::{get, get_with_debug};

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RoomTool>()
            .add_systems(EguiContextPass, RoomTool::debug_window)
        ;
    }
}

#[derive(Resource)]
struct RoomTool {
    debug_window: bool,
    last_min: Vec3,
    last_max: Vec3,
    active_min: Option<Vec3>,
    active_max: Option<Vec3>,
}

impl Default for RoomTool {
    fn default() -> Self {
        Self {
            debug_window: true,
            last_min: Vec3::ZERO,
            last_max: Vec3::new(10., 10., 10.),
            active_min: None,
            active_max: None,
        }
    }
}

impl RoomTool {
    fn clear(&mut self) {
        self.active_min = None;
        self.active_max = None;
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
    
    fn debug_window(
        mut contexts: EguiContexts,
        tool: Res<RoomTool>,
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
        });
    }
}

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
