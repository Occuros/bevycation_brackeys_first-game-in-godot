use bevy::app::App;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::player::components::*;
use crate::player::systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<InputMap<PlayerAction>>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, player_setup_system)
            .add_systems(Update, spawn_player_system)
            .add_systems(Update, spawn_player_at_start_system)
            .add_systems(Update, spawn_player_on_input_system)
        ;
    }
}