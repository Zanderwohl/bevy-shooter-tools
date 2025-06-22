use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext, EguiContextPass, EguiContexts};
use bevy_egui::egui::Ui;
use strum_macros::Display;
use crate::editor::multicam::MulticamState;


pub struct EditorPanelPlugin;
impl Plugin for EditorPanelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EditorPanels>()
            .add_systems(Startup, EditorPanels::set_multicam_size)
            .add_systems(EguiContextPass, EditorPanels::ui)
        ;
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Display)]
pub enum EditorPanelLocation {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Resource)]
pub struct EditorPanels {
    toolbar_height: f32,
    top_height: f32,
    bottom_height: f32,
    left_width: f32,
    right_width: f32,
}

pub enum PanelError {
    PanelWithKeyAlreadyExists(String),
    SectionDoesNotExist(String),
    PanelDoesNotExist(String),
}

impl Default for EditorPanels {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorPanels {
    pub fn new() -> Self {
        Self {
            toolbar_height: 20.0,
            top_height: 40.0,
            bottom_height: 30.0,
            left_width: 40.0,
            right_width: 40.0,
        }
    }

    fn ui(mut panels: ResMut<Self>, mut contexts: EguiContexts, multicam_state: ResMut<MulticamState>, windows: Query<&Window, With<PrimaryWindow>>) -> Result{
        let ctx = contexts.try_ctx_mut();
        if ctx.is_none() {
            return Ok(());
        }
        let ctx = ctx.unwrap();

        panels.top_height = egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(ui);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();
        panels.left_width = egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(ui);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        panels.right_width = egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(ui);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        panels.bottom_height = egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(ui);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();

        Self::set_multicam_size(panels, multicam_state, windows)
    }

    fn ui_for_panel(ui: &mut Ui) {
        ui.label("Panel is empty.");
    }

    fn set_multicam_size(panels: ResMut<Self>, mut multicam_state: ResMut<MulticamState>, windows: Query<&Window, With<PrimaryWindow>>,) -> Result {
        let window = windows.single()?;

        let left_taken = panels.left_width / window.width();
        let right_taken = panels.right_width / window.width();
        let bottom_taken = panels.bottom_height / window.height();
        let top_taken = (panels.toolbar_height + panels.top_height) / window.height();
        // info!("[{} {}] -> [{}, {}]", left_taken, top_taken, 1.0 - right_taken, 1.0 - bottom_taken);

        multicam_state.start = Vec2::new(left_taken, top_taken);
        multicam_state.end = Vec2::new(1.0 - right_taken, 1.0 - bottom_taken);

        Ok(())
    }
}