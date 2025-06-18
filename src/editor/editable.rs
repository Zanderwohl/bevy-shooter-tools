use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use crate::common::cuboid::CuboidPoint;
use crate::common::PointResolutionError;

lazy_static! {
    static ref MAP_EXT: String = "gmp".to_owned(); // Grackle MaP
    static ref MAP_ART: String = "gma".to_owned(); // Grackle Map Artifact
}



#[typetag::serde]
pub trait EditorObject: Send + Sync {
    fn get_point(&self, key: &str) -> Result<Vec3, PointResolutionError>;
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Copy)]
#[derive(Hash)]
pub struct EditorActionId {
    _id: u64,
}

#[derive(Resource)]
pub struct EditorActions {
    actions: HashMap<EditorActionId, EditorAction>,
    id_counter: u64,
}

impl Default for EditorActions {
    fn default() -> Self {
        Self {
            actions: HashMap::new(),
            id_counter: 0,
        }
    }
}

impl EditorActions {
    fn next_id(&mut self) -> EditorActionId {
        let id = EditorActionId { _id: self.id_counter };
        self.id_counter += 1;
        id
    }
    
    pub fn take_action(&mut self, object: Box<dyn EditorObject>) {
        let new_action = EditorAction {
            id: self.next_id(),
            object,
            parents: vec![],
        };
        self.actions.insert(new_action.id, new_action);
    }
    
    pub fn get_action(&self, id: &EditorActionId) -> Option<&EditorAction> {
        self.actions.get(id)
    }
}

impl EditorActionId {
    pub fn new() -> Self {
        EditorActionId { _id: 0 }
    }
}

// Essentially, these are nodes on a directed acyclic graph.
// Editor actions may depend on previous actions to resolve.
// They may also not depend on anything, with all descendants tracing their ancestry back --
// such a case would lead to multiple disjoint graphs, which is okay.
#[derive(Serialize, Deserialize)]
pub struct EditorAction {
    id: EditorActionId,
    object: Box<dyn EditorObject>,
    parents: Vec<EditorActionId>,
}

impl EditorAction {
    pub fn get_point(&self, key: &str) -> Result<Vec3, PointResolutionError> {
        self.object.get_point(key)
    }
}

pub struct RefVec3 {
    x: Ref32,
    y: Ref32,
    z: Ref32,
}

pub enum Ref32 {
    Absolute(f32),
    Relative(EditorActionId, CuboidPoint, f32),
}

impl Ref32 {
    pub fn resolve(&self, actions: ResMut<EditorActions>) -> Result<f32, PointResolutionError> {
        match self {
            Ref32::Absolute(f) => Ok(*f),
            Ref32::Relative(id, p, f) => {
                let action = actions.get_action(id).ok_or(PointResolutionError::NoSuchReferent)?;
                todo!()
            }
        }
    }
}
