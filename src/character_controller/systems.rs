use bevy::prelude::*;
use bevy_xpbd_2d::components::{Rotation, Sleeping};
use bevy_xpbd_2d::math::{AdjustPrecision, Vector};
use bevy_xpbd_2d::prelude::{LinearVelocity, ShapeHits};
use leafwing_input_manager::action_state::ActionState;
use crate::character_controller::components::{AirAcceleration, AirDampingFactor, CharacterController, Grounded, JumpImpulse, MaxSlopeAngle, MovementAcceleration, MovementDampingFactor};
use crate::Player;
use crate::player::components::PlayerAction;
use crate::world::components::{IsDead, PassThroughOneWayPlatform};



/// Updates the [`Grounded`] status for character controllers.
pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
pub fn movement(
    time: Res<Time>,
    player_actions_query: Query<&ActionState<PlayerAction>>,
    mut controllers: Query<(
        &MovementAcceleration,
        &AirAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    ), Without<IsDead>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    let Ok(input) = player_actions_query.get_single() else { return };
    // for event in movement_event_reader.read() {
    for (movement_acceleration, air_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
        &mut controllers
    {
        if input.pressed(&PlayerAction::Move) {
            let direction = input.clamped_axis_pair(&PlayerAction::Move).unwrap().x();
            if is_grounded {
                linear_velocity.x += direction * movement_acceleration.0 * delta_time;
            } else {
                linear_velocity.x += direction * air_acceleration.0 * delta_time;
            }
        }

        if input.just_pressed(&PlayerAction::Jump) {
            if is_grounded {
                linear_velocity.y = jump_impulse.0;
            }
        }
    }
}

/// Slows down movement in the X direction.
pub fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &AirDampingFactor, &mut LinearVelocity, Has<Grounded>)>) {
    for (damping_factor, air_damping_factor, mut linear_velocity, is_grounded) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        if is_grounded {
            linear_velocity.x *= damping_factor.0;
        } else {
            linear_velocity.x *= air_damping_factor.0;
        }
    }
}


pub fn activate_pass_through_one_way_platform_system(
    mut commands: Commands,
    player_actions_query: Query<&ActionState<PlayerAction>>,
    mut players: Query<(Entity, &mut PassThroughOneWayPlatform), With<Player>>,
) {
    let Ok(input) = player_actions_query.get_single() else { return };

    for (entity, mut pass_through_one_way_platform) in &mut players {
        if input.just_pressed(&PlayerAction::DropDown) {
            *pass_through_one_way_platform = PassThroughOneWayPlatform::Always;
            // Wake up body when it's allowed to drop down.
            // Otherwise it won't fall because gravity isn't simulated.
            commands.entity(entity).remove::<Sleeping>();
        } else {
            *pass_through_one_way_platform = PassThroughOneWayPlatform::ByNormal;
        }
    }
}
