mod character_controller;
mod world;
mod player;
mod debugging;
mod enemy;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::utils::HashSet;
use bevy_ecs_ldtk::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_spritesheet_animation::prelude::SpritesheetAnimationPlugin;
use bevy_vector_shapes::prelude::*;
use bevy_xpbd_2d::prelude::*;
use crate::character_controller::CharacterControllerPlugin;
use crate::debugging::DebuggingPlugin;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::world::components::{GameSounds, IsDead};
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .register_type::<EntityInstance>()
        .register_type::<HashSet<Entity>>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(SpritesheetAnimationPlugin)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(EditorPlugin::default())
        .add_plugins(DebuggingPlugin)
        .add_plugins(CharacterControllerPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, zoom_scale_system)
        .add_systems(PostUpdate, camera_follow_player_system.after(PhysicsSet::Sync).before(TransformSystem::TransformPropagate))
        .insert_resource(Msaa::Off)

        .run();
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct Player;

#[derive(Component, Reflect, Default)]
pub struct Inventory {
    pub collected_coins: i32,
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_sounds: Res<GameSounds>,
) {
    let ldtk_handle = asset_server.load("first_game.ldtk");
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle,
            transform: Transform::from_xyz(50.0, 100.0, -10.0),
            ..Default::default()
        },
        Name::new("World")
    ));

    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));

    //background music
    commands.spawn((
        Name::new("Background Music"),
        AudioBundle {
            source: game_sounds.background_music.clone(),
            ..default()
        }));
}

pub fn camera_follow_player_system(
    q_player: Query<&Transform, (With<Player>, Without<IsDead>)>,
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = q_player.get_single() else { return; };
    let mut camera_transform = q_camera.single_mut();

    camera_transform.translation = player_transform.translation + Vec3::Y * 50.0;
}

fn zoom_scale_system(
    mut query_camera: Query<&mut OrthographicProjection, Added<MainCamera>>,
) {
    //if we scale the projection at creation, playersprite won't be rendered
    for mut projection in query_camera.iter_mut() {
        projection.scale = 0.4;
    }
}

