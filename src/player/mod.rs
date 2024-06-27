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
            .init_resource::<PlayerAnimations>()
            .add_event::<CoinCollected>()
            .register_type::<InputMap<PlayerAction>>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, setup_player_input_system)
            .add_systems(Update, spawn_player_system)
            .add_systems(Update, spawn_player_at_start_system)
            .add_systems(Update, restart_level_on_input_system)
            .add_systems(Update, player_animation_system)
            .add_systems(Update, coin_collection_system)
        ;
    }
}