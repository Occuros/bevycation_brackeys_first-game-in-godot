mod character_controller;
mod world;
mod player;
mod debugging;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::utils::tracing::Instrument;
use bevy_ecs_ldtk::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_vector_shapes::prelude::*;
use bevy_xpbd_2d::math::Scalar;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::character_controller::CharacterControllerPlugin;
use crate::character_controller::components::*;
use crate::debugging::DebuggingPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .register_type::<EntityInstance>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LdtkPlugin)
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
        .add_plugins(PlayerPlugin )
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, zoom_scale_system)
        .add_systems(PostUpdate, camera_follow_player_system.after(PhysicsSet::Sync).before(TransformSystem::TransformPropagate))

        .run();
}

#[derive(Component, Reflect)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct Player;


fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // painter: ShapeCommands,
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




    // painter.transform.translation.y -= 10.0;
    // painter.rect(Vec2::new(500.0, 20.0))
    //     .insert(Name::new("Floor"))
    //     .insert(Transform::from_translation(-Vec3::Y * 100.0))
    //     .insert(Collider::rectangle(500.0, 20.0))
    //     .insert(RigidBody::Static);
}

pub fn camera_follow_player_system(
    q_player: Query<&Transform, With<Player>>,
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

