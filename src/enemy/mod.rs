use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::enemy::components::*;
use crate::enemy::systems::*;

mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SlimeAnimations>()
            .add_systems(Update, setup_enemy_system)
            .add_systems(Update, enemy_wall_detection_system)
            .add_systems(Update, enemy_movement_system)
            .register_ldtk_entity::<EnemyBundle>("Slime")

        ;
    }
}
