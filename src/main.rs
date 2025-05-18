mod common;
mod startup;
mod editor;

use bevy::prelude::*;
use bevy::window::{ExitCondition, PresentMode};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};

use crate::common::lang::change_lang;
use crate::common::perf::PerfPlugin;
use crate::editor::multicam::MulticamPlugin;

fn main() {
    let editor_params = startup::EditorParams::new()
        .unwrap_or_else(|message| {
            eprintln!("Editor Startup Error:\n{}", message);
            std::process::exit(1);
        });
    change_lang(&editor_params.lang)
        .unwrap_or_else(|message| {
            eprintln!("Language map error:\n{}", message);
            std::process::exit(1);
        });

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
               primary_window: Some(Window {
                   title: get!("editor.title"),
                   name: Some("bevy-shooter-tools.app".to_owned()),
                   present_mode: PresentMode::AutoVsync,
                   prevent_default_event_handling: true,
                   visible: true,
                   ..default()
               }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            }),
        )
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins((
            MulticamPlugin {
                test_scene: true,
            },
            PerfPlugin,
            ))
        .run();
}
