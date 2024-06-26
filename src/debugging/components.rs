use std::fmt::Debug;
use bevy::prelude::{KeyCode, Reflect};
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::input_map::InputMap;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum DebugAction {
    ShowColliders
}

impl DebugAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(DebugAction::ShowColliders, KeyCode::KeyK);


        return input_map;
    }
}