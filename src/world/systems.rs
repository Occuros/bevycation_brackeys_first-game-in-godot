use bevy::prelude::*;
use bevy_ecs_ldtk::{TileEnumTags};
use bevy_xpbd_2d::math::{Scalar, Vector};
use bevy_xpbd_2d::prelude::*;
use crate::{Player};
use crate::world::components::*;




pub(crate) fn add_colliders_to_walls(
    mut commands: Commands,
    wall_query: Query<Entity, (Added<Wall>, Without<Collider>)>,
) {
    for entity in wall_query.iter() {
        commands.entity(entity)
            .insert(Name::new("Wall"))
            .with_children(|commands| {
                commands.spawn((
                    TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
                    Collider::rectangle(16.0, 16.0),
                    RigidBody::Static,
                ));
            });
    }
}


pub fn add_colliders_to_platforms(
    mut commands: Commands,
    platform_query: Query<Entity, (Added<Platform>, Without<Collider>)>,
) {
    for entity in platform_query.iter() {
        commands.entity(entity)
            .insert(RigidBody::Kinematic)
            .insert(OneWayPlatform::default())
            .with_children(|commands| {
                commands.spawn((
                    Name::new("PlatformCollider"),
                    TransformBundle::from_transform(Transform::from_xyz(0.0, 4.0, 0.0)),
                    Collider::rectangle(32.0, 8.0),
                ));
            });
    }
}


pub fn add_colliders_to_bridges(
    mut commands: Commands,
    platform_query: Query<(Entity, &TileEnumTags), (Added<Bridge>, Without<Collider>)>,
) {
    for (entity, enum_tag) in platform_query.iter() {
        // let radius = 3.5;
        commands.entity(entity)
            .insert(Name::new("Bridge"))
            .insert(RigidBody::Kinematic)
            .insert(Friction::new(1.0))
            .insert(OneWayPlatform::default())
            .with_children(|commands| {
                if enum_tag.tags.contains(&"StartBridge".to_string()) {
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(0.0, 4.2, 0.0)
                            .with_rotation(Quat::from_rotation_z(-0.2))),
                        Collider::rectangle(16.0, 4.0)
                    ));

                } else if enum_tag.tags.contains(&"MiddleBridge".to_string()) {
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(0.0, 2.5, 0.0)),
                        Collider::rectangle(16.0, 4.0)
                    ));
                } else if enum_tag.tags.contains(&"EndBridge".to_string()) {
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(0.0, 4.2, 0.0)
                            .with_rotation(Quat::from_rotation_z(0.2))),
                        Collider::rectangle(16.0, 4.0)
                    ));
                }
            });
    }
}



pub fn one_way_platform_system(
    mut one_way_platforms_query: Query<&mut OneWayPlatform>,
    other_colliders_query: Query<
        Option<&PassThroughOneWayPlatform>,
        (With<Collider>, Without<OneWayPlatform>), // NOTE: This precludes OneWayPlatform passing through a OneWayPlatform
    >,
    mut collisions: ResMut<Collisions>,
    collision_parent: Query<&ColliderParent>,
) {
    // This assumes that Collisions contains empty entries for entities
    // that were once colliding but no longer are.
    collisions.retain(|contacts| {
        // This is used in a couple of if statements below; writing here for brevity below.
        fn any_penetrating(contacts: &Contacts) -> bool {
            contacts.manifolds.iter().any(|manifold| {
                manifold
                    .contacts
                    .iter()
                    .any(|contact| contact.penetration > 0.0)
            })
        }

        // Differentiate between which normal of the manifold we should use
        enum RelevantNormal {
            Normal1,
            Normal2,
        }
        let entity1 = collision_parent.get(contacts.entity1).map(|p| p.get()).unwrap_or_else(|_| contacts.entity1);
        let entity2 = collision_parent.get(contacts.entity2).map(|p| p.get()).unwrap_or_else(|_| contacts.entity2);

        // First, figure out which entity is the one-way platform, and which is the other.
        // Choose the appropriate normal for pass-through depending on which is which.
        let (mut one_way_platform, other_entity, relevant_normal) =
            if let Ok(one_way_platform) = one_way_platforms_query.get_mut(entity1) {
                (one_way_platform, entity2, RelevantNormal::Normal1)
            } else if let Ok(one_way_platform) = one_way_platforms_query.get_mut(entity2) {
                (one_way_platform, entity1, RelevantNormal::Normal2)
            } else {
                // Neither is a one-way-platform, so accept the collision:
                // we're done here.
                return true;
            };

        if one_way_platform.0.contains(&other_entity) {
            // If we were already allowing a collision for a particular entity,
            // and if it is penetrating us still, continue to allow it to do so.
            if any_penetrating(contacts) {
                return false;
            } else {
                // If it's no longer penetrating us, forget it.
                one_way_platform.0.remove(&other_entity);
            }
        }

        match other_colliders_query.get(other_entity) {
            // Pass-through is set to never, so accept the collision.
            Ok(Some(PassThroughOneWayPlatform::Never)) => true,
            // Pass-through is set to always, so always ignore this collision
            // and register it as an entity that's currently penetrating.
            Ok(Some(PassThroughOneWayPlatform::Always)) => {
                one_way_platform.0.insert(other_entity);
                false
            }
            // Default behaviour is "by normal".
            Err(_) | Ok(None) | Ok(Some(PassThroughOneWayPlatform::ByNormal)) => {
                // If all contact normals are in line with the local up vector of this platform,
                // then this collision should occur: the entity is on top of the platform.
                if contacts.manifolds.iter().all(|manifold| {
                    let normal = match relevant_normal {
                        RelevantNormal::Normal1 => manifold.normal1,
                        RelevantNormal::Normal2 => manifold.normal2,
                    };

                    normal.length() > Scalar::EPSILON && normal.dot(Vector::Y) >= 0.5
                }) {
                    true
                } else if any_penetrating(contacts) {
                    // If it's already penetrating, ignore the collision and register
                    // the other entity as one that's currently penetrating.
                    one_way_platform.0.insert(other_entity);
                    false
                } else {
                    // In all other cases, allow this collision.
                    true
                }
            }
        }
    });
}

pub fn move_platforms_system(
    mut platform_query: Query<(&mut Transform, &mut LinearVelocity, &mut Path), With<Platform>>
) {
    for (mut transform, mut linvel, mut path) in platform_query.iter_mut() {
        if path.points.len() <= 1 { continue; };

        let next_point = path.points[path.index];
        let mut new_velocity =
            (next_point - transform.translation.truncate()).normalize() * path.speed;

        if new_velocity.dot(linvel.0) < 0. {
            if path.index == 0 {
                path.forward = true;
            } else if path.index == path.points.len() - 1 {
                path.forward = false;
            }

            transform.translation.x = path.points[path.index].x;
            transform.translation.y = path.points[path.index].y;

            if path.forward {
                path.index += 1;
            } else {
                path.index -= 1;
            }

            new_velocity =
                (path.points[path.index] - transform.translation.truncate()).normalize() * path.speed;
        }

        linvel.0 = new_velocity;
    }
}

pub fn kill_zone_system(
    mut commands: Commands,
    kill_zone_query: Query<&CollidingEntities, With<KillZone>>,
    player_query: Query<Entity, With<Player>>
) {
    for collisions in kill_zone_query.iter() {
        for other in collisions.iter() {
            if player_query.contains(*other) {
                commands.entity(*other).despawn_recursive();
            }
        }
    }
}