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
        let selected_color = Color::srgb_u8(0, 255, 0);
        let hovered_color = Color::srgb_u8(230, 230, 230);
        let same_color = Color::srgb_u8(230, 230, 0);
        for (entity, transform, select) in selectables {
            let same = match (state.selected, state.hovered) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            };
            if let Some(selected) = state.selected {
                let color = if same { same_color } else { selected_color };
                if selected == entity {
                    Self::draw_bounding_box(&mut gizmos, color, transform, select);
                }
            }
            if let Some(hovered) = state.hovered {
                if hovered == entity {
                    Self::draw_bounding_box(&mut gizmos, hovered_color, transform, select);
                }
            }
        }
    }
    
    fn local_to_world(transform: &Transform, point: &Vec3) -> Vec3 {
        transform.transform_point(*point)
    }

    fn draw_bounding_box(gizmos: &mut Gizmos, color: Color, transform: &Transform, select: &EditorSelectable) {
        let a = transform.transform_point(Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z));
        let b = transform.transform_point(Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z));
        let c = transform.transform_point(Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z));
        let d = transform.transform_point(Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(select.bounding_box.half_size.z));

        let e = transform.transform_point(Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z));
        let f = transform.transform_point(Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z));
        let g = transform.transform_point(Vec3::ZERO.with_x(-select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z));
        let h = transform.transform_point(Vec3::ZERO.with_x(select.bounding_box.half_size.x).with_y(-select.bounding_box.half_size.y).with_z(-select.bounding_box.half_size.z));

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
    pub hovered: Option<Entity>,
    pub selected: Option<Entity>,
}
