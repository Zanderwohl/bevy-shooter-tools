use bevy::app::App;
use bevy::math::bounding::Bounded3d;
use bevy::prelude::*;


pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SelectionEvent>()
            .init_resource::<SelectionState>()
            .add_systems(Update, (
                Self::select,
                Self::draw_bounds,
            ))
        ;
    }
}

impl SelectionPlugin {
    fn select(
        mut selection_events: EventReader<SelectionEvent>,
        mut state: ResMut<SelectionState>,
    ) {
        for selection in selection_events.read() {
            state.selected = selection.id;
        }
    }

    fn draw_bounds(
        selectables: Query<(Entity, &Transform, &EditorSelectable)>,
        state: Res<SelectionState>,
        mut gizmos: Gizmos,
    ) {
        let color = Color::srgb_u8(0, 255, 0);
        for (entity, transform, select) in selectables {
            if let Some(selected) = state.selected {
                if selected == entity {
                    // Draw box from select.bounding_box using the gizmos
                    let a = transform.translation + Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z);
                    let b = transform.translation + Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z);
                    let c = transform.translation + Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z);
                    let d = transform.translation + Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z);

                    let e = transform.translation + Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z);
                    let f = transform.translation + Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z);
                    let g = transform.translation + Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z);
                    let h = transform.translation + Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z);
                    
                    gizmos.line(a, b, color);
                    gizmos.line(b, c, color);
                    gizmos.line(c, d, color);
                    gizmos.line(d, a, color);

                    gizmos.line(e, f, color);
                    gizmos.line(f, g, color);
                    gizmos.line(g, h, color);
                    gizmos.line(h, e, color);

                    gizmos.line(a, e, color);
                    gizmos.line(b, f, color);
                    gizmos.line(c, g, color);
                    gizmos.line(d, h, color);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct EditorSelectable {
    pub id: String,
    pub bounding_box: Cuboid,
}

#[derive(Event)]
pub struct SelectionEvent {
    pub id: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct SelectionState {
    selected: Option<Entity>,
}
