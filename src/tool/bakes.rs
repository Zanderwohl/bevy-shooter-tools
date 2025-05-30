use bevy::prelude::*;
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use crate::get;
use crate::tool::room::RecalculateRoomIntersections;

pub struct BakePlugin;

impl Plugin for BakePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(EguiContextPass, Self::bake_ui)
        ;
    }
}

impl BakePlugin {
    fn bake_ui(
        mut contexts: EguiContexts,
        mut room_events: EventWriter<RecalculateRoomIntersections>
    ) {
        let ctx = contexts.try_ctx_mut();
        if ctx.is_none() { return; }
        let ctx = ctx.unwrap();
        
        egui::Window::new(get!("bakes.title")).show(ctx, |ui| {
           ui.vertical(|ui| {
               if ui.button(get!("bakes.intersections")).clicked() {
                   room_events.write(RecalculateRoomIntersections);
               }
           });
        });
    }
}
