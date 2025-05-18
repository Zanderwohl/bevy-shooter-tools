use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use iyes_perf_ui::PerfUiPlugin;
use bevy::prelude::{AppExtStates, Commands, Component, KeyCode, NextState, OnEnter, Res, ResMut, State, States};
use iyes_perf_ui::entries::{PerfUiFixedTimeEntries, PerfUiFramerateEntries, PerfUiWindowEntries};
use bevy::input::ButtonInput;

pub struct PerfPlugin;

#[derive(Component)]
struct PerfUI;

impl Plugin for PerfPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(DebugState::Off)
            .add_plugins(FrameTimeDiagnosticsPlugin::new(256))
            .add_plugins(PerfUiPlugin)
            .add_systems(OnEnter(DebugState::Off), crate::common::systems::despawn_recursive_entities_with::<PerfUI>)
            .add_systems(OnEnter(DebugState::AllPerf), add_all_perf)
            .add_systems(Update, toggle_perf)
            .init_state::<DebugState>()
        ;
    }
}

fn add_all_perf(mut commands: Commands) {
    commands.spawn((
        PerfUI,
        PerfUiFramerateEntries::default(),
        PerfUiWindowEntries::default(),
        PerfUiFixedTimeEntries::default(),
    ));
}

fn toggle_perf(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<DebugState>>,
    mut next_state: ResMut<NextState<DebugState>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        match state.get() {
            DebugState::Off => {
                next_state.set(DebugState::AllPerf);
            },
            DebugState::AllPerf => {
                next_state.set(DebugState::Off);
            },
        }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum DebugState {
    #[default]
    Off,
    AllPerf,
}
