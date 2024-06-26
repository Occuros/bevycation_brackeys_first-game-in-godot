mod components;
mod systems;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;
use crate::debugging::components::DebugAction;
use crate::debugging::systems::*;

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(InputManagerPlugin::<DebugAction>::default())

            .add_systems(Startup, setup_debugging_system)
            .add_systems(Update, active_physics_debug_system)
        ;
    }
}



