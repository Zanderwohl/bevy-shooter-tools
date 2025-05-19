use bevy::app::App;
use bevy::prelude::{Plugin, ResMut, Resource};
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ToolState>()
            .add_systems(EguiContextPass, Self::toolbar)
        ;
    }
}

#[derive(Resource)]
pub struct ToolState {
    current: Tools,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            current: Tools::default(),
        }
    }
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Default, EnumIter)]
pub enum Tools {
    #[default]
    Select,
    Move,
}

impl ToolPlugin {
    fn toolbar(mut contexts: EguiContexts, mut tool_state: ResMut<ToolState>) {
        let ctx = contexts.ctx_mut();

        egui::Window::new("Tools").show(ctx, |ui| {
           egui::Grid::new("tools").show(ui, |ui| {
               for item in Tools::iter() {
                   if tool_state.current == item {
                       ui.scope(|ui| {
                           ui.disable();
                           let _ = ui.button(item.to_string());
                       });
                   } else {
                       if ui.button(item.to_string()).clicked() {
                           tool_state.current = item;
                       }
                   }
               }
           })
        });
    }
}
