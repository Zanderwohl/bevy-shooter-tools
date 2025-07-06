use std::sync::Arc;
use grackle::{get, startup};
use grackle::common;

use bevy::prelude::*;
use bevy::window::{ExitCondition, PresentMode};
use bevy_egui::{egui, EguiContextPass, EguiContexts, EguiPlugin};
use bevy_egui::egui::ScrollArea;
use grackle::common::item::item::{Item, ParticleEffect, Prototype, StatTracker};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let editor_params = startup::EditorParams::new()
        .unwrap_or_else(|message| {
            eprintln!("Editor Startup Error:\n{}", message);
            std::process::exit(1);
        });
    common::lang::change_lang(&editor_params.lang)
        .unwrap_or_else(|message| {
            eprintln!("Language map error:\n{}", message);
            std::process::exit(1);
        });
    
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: get!("crate_drop.title"),
                    name: Some("grackle-drop-tester.app".to_owned()),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: true,
                    visible: true,
                    ..default()
                }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            })
        )
        .add_plugins((
            EguiPlugin { enable_multipass_for_primary_context: true },
        ))
        .init_resource::<State>()
        .add_systems(Startup, setup)
        .add_systems(EguiContextPass, ui)
        .run();
    ;
    Ok(())
}

fn setup(
    mut state: ResMut<State>,
) {
    let shotgun = Arc::new(Prototype {
        name_key: "shotgun".to_owned(),
        stock: true,
        trade_restriction: false,
    });
    let medigun = Arc::new(Prototype {
        name_key: "medigun".to_owned(),
        stock: true,
        trade_restriction: false,
    });
    let top_hat = Arc::new(Prototype {
        name_key: "top_hat".to_owned(),
        stock: false,
        trade_restriction: false,
    });
    let electric = Arc::new(ParticleEffect {
        name_key: "electric".to_owned(),
    });
    
    state.items.push(Item::new(shotgun.clone()));
    state.items.push(Item::new_with(shotgun.clone(), Some(StatTracker {
        kills: Some(0),
        assists: Some(0),
        damage: Some(0),
        ..Default::default()
    }), None));
    state.items.push(Item::new(medigun.clone()));
    state.items.push(Item::new_with(medigun.clone(), Some(StatTracker::default_healing()), None));
    state.items.push(Item::new_with(top_hat.clone(), None, None));
    state.items.push(Item::new_with(top_hat.clone(), Some(StatTracker::default_points()), None));
    state.items.push(Item::new_with(top_hat.clone(), Some(StatTracker::default_points()), Some(electric.clone())));
}

#[derive(Resource)]
struct State {
    items: Vec<Item>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            items: Vec::new(),
        }
    }
}

fn ui(
    mut state: ResMut<State>,
    mut contexts: EguiContexts,
) {
    let ctx = contexts.try_ctx_mut();
    if ctx.is_none() {
        return;
    }
    let ctx = ctx.unwrap();
    
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.heading(get!("crate_drop.controls.title"));
        
        if ui.button(get!("crate_drop.controls.new")).clicked() {
            
        }
    });
    
    egui::SidePanel::right("right_panel").show(ctx, |ui| {
        ui.heading(get!("crate_drop.history.title"));

        ScrollArea::vertical().show(ui, |ui| {
            for item in &state.items {
                ui.separator();
                
                ui.vertical(|ui| {
                    ui.label(item.display_name());
                    if let Some(particle_effect) = &item.particle_effect {
                        ui.label(particle_effect.name());
                    }
                    if let Some(stat_tracker) = &item.stat_tracker {
                        ui.label(get!("stat_tracker.tracks", "list", stat_tracker.tracks_list()));
                    }
                });
            }
        });
    });
}
