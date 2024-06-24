use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy_ecs_ldtk::TileEnumTags;
use bevy_xpbd_2d::math::{Scalar, Vector};
use bevy_xpbd_2d::prelude::*;
use crate::{AnimationIndices, Player};
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
            .insert(RigidBody::Static)
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


    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 3 };
    for (entity, enum_tag) in platform_query.iter() {
        info!("bridge: {:?}", enum_tag);
        let radius = 4.0;
        commands.entity(entity)
            .insert(Name::new("Bridge"))
            .insert(RigidBody::Kinematic)
            .insert(OneWayPlatform::default())
            .with_children(|commands| {
                if enum_tag.tags.contains(&"StartBridge".to_string()) {
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(-4.0, 4.5, 0.0)),
                        Collider::circle(radius),
                    ));
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(4.0, 3.5, 0.0)),
                        Collider::circle(radius),
                    ));
                }
                else if enum_tag.tags.contains(&"MiddleBridge".to_string()) {
                    // commands.spawn((
                    //     TransformBundle::from_transform(Transform::from_xyz(-4.0, 2.5, 0.0)),
                    //     Collider::circle(radius),
                    // ));
                    // commands.spawn((
                    //     TransformBundle::from_transform(Transform::from_xyz(4.0, 2.5, 0.0)),
                    //     Collider::circle(radius),
                    // ));
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(0.0, 2.5, 0.0)),
                        Collider::rectangle(16.0, 4.0)
                    ));
                } else if enum_tag.tags.contains(&"EndBridge".to_string()) {
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(-4.0, 3.5, 0.0)),
                        Collider::circle(radius),
                    ));
                    commands.spawn((
                        TransformBundle::from_transform(Transform::from_xyz(4.0, 4.5, 0.0)),
                        Collider::circle(radius),
                    ));
                }

            });
    }
}

pub fn activate_pass_through_one_way_platform_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(Entity, &mut PassThroughOneWayPlatform), With<Player>>,
) {
    for (entity, mut pass_through_one_way_platform) in &mut players {
        if (keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS)) && keyboard_input.just_pressed(KeyCode::Space) {
            *pass_through_one_way_platform = PassThroughOneWayPlatform::Always;
            // Wake up body when it's allowed to drop down.
            // Otherwise it won't fall because gravity isn't simulated.
            commands.entity(entity).remove::<Sleeping>();
        } else {
            *pass_through_one_way_platform = PassThroughOneWayPlatform::ByNormal;
        }
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