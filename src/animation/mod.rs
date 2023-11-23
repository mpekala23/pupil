use std::hash::Hash;

use bevy::{prelude::*, render::texture::ImageSampler, sprite::TextureAtlas, utils::HashMap};

use crate::agent::AgentAnimState;

// Trait defining what we need for an enum to be an animation state
pub trait Animatable: Component + Eq + Hash + Clone {}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Clone)]
/// Just be normal. Start with the leaf nodes
pub struct AnimationRoot<State> {
    pub state: State,
    pub filename: String,
    pub width: i32,
    pub height: i32,
    pub length: usize,
}

impl<State> AnimationRoot<State> {
    fn new(state: State, filename: String, width: i32, height: i32, length: usize) -> Self {
        return Self {
            state,
            filename,
            width,
            height,
            length,
        };
    }
}

#[derive(Component)]
pub struct AnimationVal<State: Animatable> {
    pub state: State,
    pub invert_x: bool,
    pub invert_y: bool,
}

#[derive(Component)]
pub struct SpriteMap<State: Animatable> {
    pub sprite_map: HashMap<State, SpriteSheetBundle>,
}

#[derive(Component)]
pub struct AnimationManager<State: Animatable> {
    pub sprite_map: HashMap<State, SpriteSheetBundle>,
    pub root_map: HashMap<State, AnimationRoot<State>>,
    timer: AnimationTimer,
}

impl<State: Animatable> AnimationManager<State> {
    pub fn new(
        roots: &Vec<AnimationRoot<State>>,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let mut sprite_map: HashMap<State, SpriteSheetBundle> = HashMap::new();
        let mut root_map: HashMap<State, AnimationRoot<State>> = HashMap::new();
        for root in roots.iter() {
            let texture_handle: Handle<Image> = asset_server.load(&root.filename);
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(root.width as f32, root.height as f32),
                root.length,
                1,
                None,
                None,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let sprite_sheet = SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                ..default()
            };
            sprite_map.insert(root.state.clone(), sprite_sheet);
            root_map.insert(root.state.clone(), root.clone());
        }
        return Self {
            sprite_map,
            root_map,
            timer: AnimationTimer(Timer::from_seconds(0.06, TimerMode::Repeating)),
        };
    }
}

#[macro_export]
macro_rules! animate_state_update {
    ($type: ty, $fname: ident) => {
        fn $fname(
            time: Res<Time>,
            mut query: Query<(
                &AnimationVal<$type>,
                &mut AnimationManager<$type>,
                &mut Handle<TextureAtlas>,
                &mut TextureAtlasSprite,
            )>,
        ) {
            for (anim_val, mut manager, mut hand, mut sprite) in &mut query {
                let cur_state = anim_val.clone();
                let cur_sheet = manager
                    .sprite_map
                    .get_mut(&cur_state.state)
                    .unwrap()
                    .clone();
                if *hand != cur_sheet.texture_atlas {
                    // The animation state has changed
                    *hand = cur_sheet.texture_atlas;
                    sprite.index = 0;
                }
                let cur_root = manager.root_map.get(&cur_state.state).unwrap();
                let length = cur_root.length;
                manager.timer.tick(time.delta());
                sprite.custom_size = Some(Vec2 { x: 64.0, y: 64.0 });
                if manager.timer.just_finished() {
                    sprite.index = if sprite.index >= length - 1 {
                        0
                    } else {
                        sprite.index + 1
                    };
                }
                sprite.flip_x = cur_state.invert_x;
                sprite.flip_y = cur_state.invert_y;
            }
        }
    };
}

// Do fun rust stuff to get all the animation states
animate_state_update!(AgentAnimState, agent_anim_update);

pub fn spritemap_fix(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if let Some(texture) = assets.get_mut(&handle) {
                    texture.sampler_descriptor = ImageSampler::nearest()
                }
            }
            _ => {}
        }
    }
}

pub fn register_animations(app: &mut App) {
    app.add_systems(Update, spritemap_fix);
    app.add_systems(Update, agent_anim_update);
}
