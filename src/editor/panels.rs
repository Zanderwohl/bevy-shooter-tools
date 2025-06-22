use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext, EguiContextPass, EguiContexts};
use strum_macros::Display;
use crate::editor::multicam::MulticamState;

pub trait GuiPanel: Send + Sync {
    fn name(&self) -> &str;
    fn ui(&self, ui: &mut egui::Ui);
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
}

impl EditorPanel {
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
            panel_order: vec![],
        }
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

    pub fn move_panel(&mut self, panel_key: String, location: EditorPanelLocation) -> Result<(), PanelError> {
        if !self.panel_locations.contains_key(&panel_key) {
            return Err(PanelError::PanelDoesNotExist(panel_key));
        }

        let old_location = self.panel_locations.get(&panel_key).unwrap();
        let section = self.panels.get_mut(old_location).unwrap();
        let panel = section.panels.remove(&panel_key).unwrap();
        let section = self.panels.get_mut(&location).unwrap();
        self.panel_locations.remove(&panel_key);
        self.panel_locations.insert(panel.name().to_string(), location);
        section.panels.insert(panel.name().to_string(), panel);

        Ok(())
    }

    fn ui(mut panels: ResMut<Self>, mut contexts: EguiContexts, mut multicam_state: ResMut<MulticamState>, windows: Query<&Window, With<PrimaryWindow>>,) -> Result{
        let ctx = contexts.try_ctx_mut();
        if ctx.is_none() {
            return Ok(());
        }
        let ctx = ctx.unwrap();

        panels.top_height = egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Top resizeable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();
        panels.left_width = egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Left resizable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        panels.right_width = egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Right resizable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width();
        panels.bottom_height = egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Bottom resizeable panel");
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height();

        Self::set_multicam_size(panels, multicam_state, windows)
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