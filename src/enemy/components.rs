use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_spritesheet_animation::animation::{AnimationDuration, AnimationId, AnimationRepeat};
use bevy_spritesheet_animation::library::SpritesheetLibrary;
use bevy_spritesheet_animation::prelude::Spritesheet;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;


#[derive(Copy, Clone, PartialEq, Debug, Component)]
pub struct MovementDirection(pub Direction2d);


#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    enemy: Enemy,
}


#[derive(Resource)]
pub struct SlimeAnimations {
    pub idle: AnimationId,
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for SlimeAnimations {
    fn from_world(world: &mut World) -> Self {
        let sprite_sheet = Spritesheet::new(4, 3);

        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let texture = asset_server.load("sprites/slime_green.png");
        //bevy still has issues with sprite leaking, adding just a little bit of padding helps
        //https://github.com/bevyengine/bevy/issues/1949
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0),
                                                   4, 3, Some(Vec2::new(0.0, 0.1)), None);
        let mut texture_atlas_layouts = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut library = world.get_resource_mut::<SpritesheetLibrary>().unwrap();
        let idle_clip_id = library.new_clip(|clip| {
            clip.push_frame_indices(sprite_sheet.horizontal_strip(0, 1, 4))
                .set_default_duration(AnimationDuration::PerFrame(200));

        });

        let idle_animation_id = library.new_animation(|animation| {
            animation
                .add_stage(idle_clip_id.into())
                .set_repeat(AnimationRepeat::Loop);
        });
        SlimeAnimations {
            idle: idle_animation_id,
            texture,
            layout: texture_atlas_layout,
        }
    }
}
