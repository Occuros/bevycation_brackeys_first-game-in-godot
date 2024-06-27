use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Run,
    Jump,
    DropDown,
    Respawn
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(PlayerAction::Move, DualAxis::left_stick());
        input_map.insert(PlayerAction::Jump, GamepadButtonType::South);
        input_map.insert(PlayerAction::Respawn, GamepadButtonType::Start);
        input_map.insert(PlayerAction::Respawn, GamepadButtonType::South);


        input_map.insert_chord(PlayerAction::DropDown, [
            InputKind::from(SingleAxis::negative_only(GamepadAxisType::LeftStickY, 0.3)),
            InputKind::from(GamepadButtonType::South)]
        );
        input_map.insert(PlayerAction::Move, VirtualDPad::wasd());
        input_map.insert(PlayerAction::Jump, KeyCode::Space);
        input_map.insert(PlayerAction::Respawn, KeyCode::Space);
        input_map.insert_chord(PlayerAction::DropDown, [KeyCode::Space, KeyCode::KeyS]);

        return input_map;
    }
}

#[derive(Component)]
pub struct PlayerVisual;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub idle_animation: AnimationId,
    pub run_animation: AnimationId,
    pub jump_animation: AnimationId,
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let texture = asset_server.load("sprites/knight.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 8, 8, None, None);

        let mut texture_atlas_layouts = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut library = world.get_resource_mut::<SpritesheetLibrary>().unwrap();

        let sprite_sheet = Spritesheet::new(8, 8);

        let idle_clip_id = library.new_clip(|clip| {
            clip.push_frame_indices(sprite_sheet.horizontal_strip(0, 0, 4));
        });

        let idle_animation_id = library.new_animation(|animation| {
            animation
                .add_stage(idle_clip_id.into())
                .set_repeat(AnimationRepeat::Loop);
        });

        let run_clip_id = library.new_clip(|clip| {
            clip.push_frame_indices(sprite_sheet.row(2));
            clip.push_frame_indices(sprite_sheet.row(3));
            clip.set_default_duration(AnimationDuration::PerFrame(100));
        });

        let run_animation_id = library.new_animation(|animation| {
            animation
                .add_stage(run_clip_id.into())
                .set_repeat(AnimationRepeat::Loop);
        });

        let jump_clip_id = library.new_clip(|clip| {
            clip.push_frame_indices(sprite_sheet.positions([(6, 3)]));
        });

        let jump_animation_id = library.new_animation(|animation| {
            animation
                .add_stage(jump_clip_id.into())
                .set_repeat(AnimationRepeat::Loop);
        });

        PlayerAnimations {
            texture: texture,
            layout: texture_atlas_layout,
            idle_animation: idle_animation_id,
            run_animation: run_animation_id,
            jump_animation: jump_animation_id,
        }
    }
}


#[derive(Event)]
pub struct CoinCollected {
    #[allow(dead_code)]
    pub amount_collected: i32,
    pub total_collected: i32,
}