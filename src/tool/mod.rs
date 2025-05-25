use bevy::app::App;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use crate::get;
use crate::tool::movement::MovementPlugin;
use crate::tool::room::RoomPlugin;
use crate::tool::selection::SelectionPlugin;

pub mod selection;
pub mod room;
pub mod movement;

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ToolData>()
            .init_state::<Tools>()
            .add_plugins(MovementPlugin)
            .add_plugins(SelectionPlugin)
            .add_plugins(RoomPlugin)
            .add_systems(EguiContextPass, Self::toolbar)
        ;
    }
}

#[derive(Resource)]
pub struct ToolData {
}

impl Default for ToolData {
    fn default() -> Self {
        Self {

        }
    }
}

#[derive(EnumIter, States, Debug, Display, Clone, PartialEq, Eq, Hash, Default)]
pub enum Tools {
    #[default]
    Select,
    Room,
}

impl Tools {
    fn name(&self) -> String {
        match self {
            Self::Select => get!("tools.select"),
            Self::Room => get!("tools.room"),
        }
    }
}

impl ToolPlugin {
    fn toolbar(
        mut contexts: EguiContexts,
        current_tool: Res<State<Tools>>,
        mut next_tool: ResMut<NextState<Tools>>,
    ) {
        let ctx = contexts.ctx_mut();

        egui::Window::new(get!("tools.title")).show(ctx, |ui| {
           egui::Grid::new("tools").show(ui, |ui| {
               for item in Tools::iter() {
                   if current_tool.eq(&item) {
                       ui.scope(|ui| {
                           ui.disable();
                           let _ = ui.button(item.name());
                       });
                   } else {
                       if ui.button(item.name()).clicked() {
                           next_tool.set(item);
                       }
                   }
               }
           })
        });
    }
}
