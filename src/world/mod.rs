use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::*;
use crate::world::systems::*;
use crate::world::components::*;
pub mod components;
pub mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnPlayerEvent>()
            .init_resource::<CoinAnimations>()
            .init_resource::<GameFonts>()
            .init_resource::<GameSounds>()
            .register_type::<TutorialText>()
            .add_systems(Update, add_colliders_to_walls_system)
            .add_systems(Update, add_colliders_to_platforms_system)
            .add_systems(Update, add_colliders_to_bridges_system)
            .add_systems(Update, setup_coin_system)
            .add_systems(Update, setup_tutorial_text_system)
            .add_systems(Update, setup_score_display_system)
            .add_systems(Update, move_platforms_system)
            .add_systems(Update, update_score_display_system)
            .add_systems(Update, play_pickup_sound_system)
            .add_systems(PostUpdate, kill_zone_system)
            .add_systems(PostProcessCollisions, one_way_platform_system)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Collision", 1)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Collision", 3)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Collision", 5)
            .register_ldtk_int_cell_for_layer::<WallBundle>("Collision", 6)
            .register_ldtk_int_cell_for_layer::<BridgeBundle>("Collision", 2)
            .register_ldtk_int_cell_for_layer::<BridgeBundle>("Collision", 4)
            .register_ldtk_entity::<PlatformBundle>("Platform")
            .register_ldtk_entity::<PlatformBundle>("BrownPlatform")
            .register_ldtk_entity::<PlatformBundle>("WrongPlatform")
            .register_ldtk_entity::<CoinBundle>("Coin")
            .register_ldtk_entity::<PlayerStartPointBundle>("PlayerStartPoint")
            .register_ldtk_entity::<KillZoneBundle>("KillZone")
            .register_ldtk_entity::<TutorialTextBundle>("TutorialText")
            .register_ldtk_entity::<ScoreDisplayBundle>("ScoreDisplay")


        ;
    }
}