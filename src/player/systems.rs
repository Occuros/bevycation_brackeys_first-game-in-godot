use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use bevy_xpbd_2d::math::Scalar;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::InputManagerBundle;
use crate::{Player};
use crate::character_controller::components::*;
use crate::player::components::*;
use crate::world::components::*;


pub fn player_setup_system(
    mut commands: Commands,
) {
    commands.spawn((
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        Name::new("Input")
    ));
}

pub fn spawn_player_on_input_system(
    player_query: Query<(), (With<Player>, Without<IsDead>)>,
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
    start_point_query: Query<Entity, Added<PlayerStartPoint>>,
    mut spawn_player_event: EventWriter<SpawnPlayerEvent>,
) {
    let Ok(start_point_entity) = start_point_query.get_single() else { return };
    // at start the global transform is not propagated yet
    let Ok(start_point) = helper.compute_global_transform(start_point_entity) else { return };
    spawn_player_event.send(SpawnPlayerEvent {
        translation: start_point.translation(),
    });
}


pub fn spawn_player_system(
    mut commands: Commands,
    player_query: Query<(Entity, Has<IsDead>), With<Player>>,
    mut player_spawn_event: EventReader<SpawnPlayerEvent>,
    player_animation: Res<PlayerAnimations>,
) {
    for spawn_event in player_spawn_event.read() {
        if let Ok((player_entity, is_dead)) = player_query.get_single() {
            if is_dead {
                commands.entity(player_entity).despawn_recursive();
            } else {
                return;
            }
        }

        // Use only the subset of sprites in the sheet that make up the run animation
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
            CollisionLayers::new(GamePhysicsLayer::Player, [GamePhysicsLayer::Enemy, GamePhysicsLayer::Ground, GamePhysicsLayer::KillZone])
        )).with_children(|commands| {
            commands.spawn((
                SpriteSheetBundle {
                    texture: player_animation.texture.clone_weak(),
                    atlas: TextureAtlas {
                        layout: player_animation.layout.clone_weak(),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 4.2, 0.0),
                    ..default()
                },
                SpritesheetAnimation::from_id(player_animation.idle_animation),
                PlayerVisual,
                Name::new("PlayerVisual")
            ));
        });
    }
}

pub fn player_animation_system(
    player_query: Query<Has<Grounded>, With<Player>>,
    player_animations: Res<PlayerAnimations>,
    input_query: Query<&ActionState<PlayerAction>>,
    mut player_visual_query: Query<(&mut Sprite, &mut SpritesheetAnimation), With<PlayerVisual>>,
) {
    let Ok(input) = input_query.get_single() else {return};
    let Ok(grounded) = player_query.get_single() else {return};

    let Ok((mut sprite, mut animation)) = player_visual_query.get_single_mut() else {return};
    let move_direction = input.clamped_axis_pair(&PlayerAction::Move).unwrap().x();
    if move_direction < 0.0 {
        sprite.flip_x = true
    } else if move_direction > 0.0 {
        sprite.flip_x = false
    }

    if grounded {
        if move_direction.abs() <= 0.01 {
            animation.animation_id = player_animations.idle_animation;
        } else {
            animation.animation_id = player_animations.run_animation;
        }
    } else {
        animation.animation_id = player_animations.jump_animation;
    }
}

