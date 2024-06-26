use bevy::prelude::*;
use bevy_xpbd_2d::prelude::PhysicsGizmos;
use leafwing_input_manager::InputManagerBundle;
use leafwing_input_manager::prelude::*;
use crate::debugging::components::DebugAction;

pub fn setup_debugging_system(
    mut commands: Commands,
    mut store: ResMut<GizmoConfigStore>,

) {
    let config = store.config_mut::<PhysicsGizmos>().0;
    config.enabled = false;
    commands.spawn((
        InputManagerBundle::with_map(DebugAction::default_input_map()),
        Name::new("DebugInput")
    ));
}

pub fn active_physics_debug_system(
    mut store: ResMut<GizmoConfigStore>,
    debug_actions_query: Query<&ActionState<DebugAction>>,

) {
    let Ok(input) = debug_actions_query.get_single() else {return};
    if input.just_pressed(&DebugAction::ShowColliders) {
        let config = store.config_mut::<PhysicsGizmos>().0;
        config.enabled = !config.enabled;
    }
}