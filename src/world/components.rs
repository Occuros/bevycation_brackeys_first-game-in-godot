use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_ecs_ldtk::prelude::*;

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
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
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