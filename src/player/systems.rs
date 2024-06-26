use bevy::asset::{Assets, AssetServer};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Added, Commands, default, Entity, EventReader, EventWriter, GlobalTransform, InheritedVisibility, KeyCode, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Timer, TimerMode, Transform, TransformBundle, TransformHelper, With};
use bevy_xpbd_2d::components::{CoefficientCombine, ColliderDensity, Friction, GravityScale, Restitution, Sleeping};
use bevy_xpbd_2d::math::Scalar;
use bevy_xpbd_2d::prelude::Collider;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::InputManagerBundle;
use crate::{AnimationIndices, AnimationTimer, Player};
use crate::character_controller::components::CharacterControllerBundle;
use crate::player::components::PlayerAction;
use crate::world::components::{PassThroughOneWayPlatform, PlayerStartPoint, SpawnPlayerEvent};


pub fn player_setup_system(
    mut commands: Commands,
) {
    commands.spawn((
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        Name::new("Input")
    ));
}

pub fn spawn_player_on_input_system(
    player_query: Query<(), With<Player>>,
    player_input_query: Query<&ActionState<PlayerAction>>,
    start_point_query: Query<&GlobalTransform, With<PlayerStartPoint>>,
    mut spawn_player_event: EventWriter<SpawnPlayerEvent>,
) {
    if !player_query.is_empty() {return};
    let Ok(input) = player_input_query.get_single() else {return};
    let Ok(start_point_transform) = start_point_query.get_single() else {return};
    if input.just_pressed(&PlayerAction::Respawn) {
        spawn_player_event.send(SpawnPlayerEvent {
            translation: start_point_transform.translation(),
        });
    }
}

pub fn spawn_player_at_start_system(
    helper: TransformHelper,
    start_point_query: Query<Entity, (Added<PlayerStartPoint>)>,
    mut spawn_player_event: EventWriter<SpawnPlayerEvent>,
) {
    let Ok(start_point_entity) = start_point_query.get_single() else { return };
    /// at start the global transform is not propagated yet
    let Ok(start_point) = helper.compute_global_transform(start_point_entity) else { return };
    spawn_player_event.send(SpawnPlayerEvent {
        translation: start_point.translation(),
    });
}


pub fn spawn_player_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut player_spawn_event: EventReader<SpawnPlayerEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for spawn_event in player_spawn_event.read() {
        if !player_query.is_empty() { return; }

        let texture = asset_server.load("sprites/knight.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 8, 8, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);


        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        commands.spawn((
            TransformBundle::from_transform(
                Transform::from_translation(spawn_event.translation),
            ),
            InheritedVisibility::default(),
            Name::new("Player"),
            Player,
            CharacterControllerBundle::new(Collider::capsule(5.0, 5.0)).with_movement(
                2000.0,
                1000.0,
                0.9,
                0.89,
                350.0,
                (30.0 as Scalar).to_radians(),
            ),
            Friction::new(1.0).with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(100.0),
            PassThroughOneWayPlatform::ByNormal,
        )).with_children(|commands| {
            commands.spawn((
                SpriteSheetBundle {
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                    transform: Transform::from_xyz(0.0, 4.2, 0.0),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ));
        });
    }
}


