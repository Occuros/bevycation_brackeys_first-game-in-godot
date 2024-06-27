pub mod components;
mod systems;

use bevy::{prelude::*};
use crate::character_controller::systems::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    update_grounded_system,
                    movement_system,
                    apply_movement_damping_system,
                    activate_pass_through_one_way_platform_system,
                )
                    .chain(),
            );
    }
}


