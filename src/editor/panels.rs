use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext, EguiContextPass, EguiContexts};
use bevy_egui::egui::Ui;
use strum_macros::Display;
use crate::editor::multicam::MulticamState;

pub trait GuiPanel: Send + Sync {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut egui::Ui);
}


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
    panels: HashMap<EditorPanelLocation, EditorPanel>,
    panel_locations: HashMap<String, EditorPanelLocation>,
    toolbar_height: f32,
    top_height: f32,
    bottom_height: f32,
    left_width: f32,
    right_width: f32,
}

struct EditorPanel {
    panels: HashMap<String, Box<dyn GuiPanel>>,
    panel_order: Vec<String>,
    selected_panel: usize,
}

impl EditorPanel {
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
            panel_order: vec![],
            selected_panel: 0,
        }
    }
    
    pub fn selected_panel_mut(&mut self) -> Option<&mut Box<dyn GuiPanel>> {
        if self.panels.len() == 0 {
            return None;
        }
        if self.selected_panel > self.panels.len() - 1 {
            let key = self.panel_order[0].clone();
            return Some(self.panels.get_mut(&key).unwrap())
        }
        let key = self.panel_order[self.selected_panel].clone();
        Some(self.panels.get_mut(&key).unwrap())
    }
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
        let mut panels = HashMap::new();
        panels.insert(EditorPanelLocation::Left, EditorPanel::new());
        panels.insert(EditorPanelLocation::Right, EditorPanel::new());
        panels.insert(EditorPanelLocation::Bottom, EditorPanel::new());
        panels.insert(EditorPanelLocation::Top, EditorPanel::new());

        Self {
            panels,
            panel_locations: HashMap::new(),
            toolbar_height: 20.0,
            top_height: 40.0,
            bottom_height: 30.0,
            left_width: 40.0,
            right_width: 40.0,
        }
    }

    pub fn add_panel(&mut self, panel: Box<dyn GuiPanel>, location: EditorPanelLocation) -> Result<(), PanelError> {
        let panel_name = panel.name().to_owned();
        if self.panel_locations.contains_key(panel.name()) {
            return Err(PanelError::PanelWithKeyAlreadyExists(panel_name));
        }
        let section = self.panels.get_mut(&location).ok_or(PanelError::SectionDoesNotExist(location.to_string()))?;

        section.panels.insert(panel_name.clone(), panel);
        self.panel_locations.insert(panel_name, location);

        Ok(())
    }

    fn ui(mut panels: ResMut<Self>, mut contexts: EguiContexts, multicam_state: ResMut<MulticamState>, windows: Query<&Window, With<PrimaryWindow>>,) -> Result{
        let ctx = contexts.try_ctx_mut();
        if ctx.is_none() {
            return Ok(());
        }
        let ctx = ctx.unwrap();

        panels.top_height = egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(&mut panels, ui, &EditorPanelLocation::Top);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();
        panels.left_width = egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(&mut panels, ui, &EditorPanelLocation::Left);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        panels.right_width = egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(&mut panels, ui, &EditorPanelLocation::Right);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        panels.bottom_height = egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                Self::ui_for_panel(&mut panels, ui, &EditorPanelLocation::Bottom);
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();

        Self::set_multicam_size(panels, multicam_state, windows)
    }

    fn ui_for_panel(mut panels: &mut ResMut<EditorPanels>, ui: &mut Ui, panel_location: &EditorPanelLocation) {
        let panel = panels.panels.get_mut(panel_location).unwrap();
        let active_panel = panel.selected_panel_mut();
        match active_panel {
            None => {
                ui.label("Panel is empty.");
            }
            Some(active_panel) => {
                active_panel.ui(ui);
            }
        }
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