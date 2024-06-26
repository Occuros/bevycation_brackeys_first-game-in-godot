use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_pixel_coords_to_translation_pivoted;
use bevy_xpbd_2d::prelude::*;

#[derive(PhysicsLayer)]
pub enum GamePhysicsLayer {
    Player, // Layer 0
    Enemy,  // Layer 1
    Ground, // Layer 2
    KillZone,
    Dead,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct IsDead;


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Bridge;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct BridgeBundle {
    pub bridge: Bridge,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Platform;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlatformBundle {
    platform: Platform,
    #[ldtk_entity]
    path: Path,
    #[sprite_sheet_bundle(no_grid)]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct OneWayPlatform(pub HashSet<Entity>);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub enum PassThroughOneWayPlatform {
    #[default]
    /// Passes through a `OneWayPlatform` if the contact normal is in line with the platform's local-space up vector
    ByNormal,
    /// Always passes through a `OneWayPlatform`, temporarily set this to allow an actor to jump down through a platform
    Always,
    /// Never passes through a `OneWayPlatform`
    Never,
}


#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Path {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
    pub speed: f32,
}

impl LdtkEntity for Path {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>,
    ) -> Path {
        let mut points = Vec::new();
        let start_point = ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        );

        points.push(start_point);

        let ldtk_path_points = entity_instance
            .iter_points_field("path")
            .expect("path field should be correctly typed");


        for ldtk_point in ldtk_path_points {
            let pixel_coords = (ldtk_point.as_vec2())
                * Vec2::splat(layer_instance.grid_size as f32);

            points.push(ldtk_pixel_coords_to_translation_pivoted(
                pixel_coords.as_ivec2(),
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot,
            ));
        }
        let speed = entity_instance.get_float_field("speed")
            .expect("speed field missing");
        Path {
            points,
            index: 1,
            forward: true,
            speed: *speed,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Coin;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CoinBundle {
    pub coin: Coin,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct PlayerStartPoint;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerStartPointBundle {
    player_start_point: PlayerStartPoint,
}

#[derive(Event)]
pub struct SpawnPlayerEvent {
    pub translation: Vec3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct KillZone;

#[derive(Clone, Default, Bundle)]
pub struct KillZoneBundle {
    pub kill_zone: KillZone,
    rigid_body: RigidBody,
    collider: Collider,
    sensor: Sensor,
}

impl LdtkEntity for KillZoneBundle {
    fn bundle_entity(_: &EntityInstance,
                     layer_instance: &LayerInstance,
                     _: Option<&Handle<Image>>,
                     _: Option<&TilesetDefinition>,
                     _: &AssetServer,
                     _:
                     &mut Assets<TextureAtlasLayout>) -> Self {
        KillZoneBundle {
            sensor: Sensor,
            kill_zone: KillZone,
            collider: Collider::rectangle(layer_instance.grid_size as f32, layer_instance.grid_size as f32),
            rigid_body: RigidBody::Static
        }
    }
}