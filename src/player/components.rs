use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Run,
    Jump,
    DropDown,
    Respawn
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(PlayerAction::Move, DualAxis::left_stick());
        input_map.insert(PlayerAction::Jump, GamepadButtonType::South);
        input_map.insert(PlayerAction::Respawn, GamepadButtonType::Start);
        input_map.insert(PlayerAction::Respawn, GamepadButtonType::South);


        input_map.insert_chord(PlayerAction::DropDown, [
            InputKind::from(SingleAxis::negative_only(GamepadAxisType::LeftStickY, 0.3)),
            InputKind::from(GamepadButtonType::South)]
        );
        input_map.insert(PlayerAction::Move, VirtualDPad::wasd());
        input_map.insert(PlayerAction::Jump, KeyCode::Space);
        input_map.insert(PlayerAction::Respawn, KeyCode::Space);
        input_map.insert_chord(PlayerAction::DropDown, [KeyCode::Space, KeyCode::KeyS]);

        return input_map;
    }
}