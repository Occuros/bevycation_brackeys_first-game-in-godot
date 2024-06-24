use bevy::prelude::*;
use bevy_xpbd_2d::prelude::LinearVelocity;
use crate::Player;

pub fn flip_player_based_on_movement(
    player_query: Query<(Entity, &LinearVelocity), With<Player>>,
    children: Query<&Children>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let Ok((player_entity, linear_velocity)) = player_query.get_single() else {return};

    for c in children.iter_descendants(player_entity) {
        let Ok(mut sprite) = sprite_query.get_mut(c) else {continue};
        if linear_velocity.x < 0.0 {
            sprite.flip_x = true
        } else if linear_velocity.x > 0.0 {
            sprite.flip_x = false
        }

    }
}