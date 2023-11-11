use bevy::{prelude::*, sprite::TextureAtlas};

/// At a high level what do we want this abstraction to be?
/// The "leaf node" should be something like a filename and length (I like the idea of each file being a separate animation)
/// However what we really want to be able to do is:
/// - Say these are the possible states that a thing can be in
/// -
struct Ignore {}

/// Just be normal. Start with the leaf nodes
struct AnimationRoot {
    filename: String,
    width: i32,
    height: i32,
    length: usize,
}

impl AnimationRoot {
    fn new(filename: String, width: i32, height: i32, length: usize) -> Self {
        return Self {
            filename,
            width,
            height,
            length,
        };
    }
}

struct AnimationBundle {
    sprite_sheet: SpriteSheetBundle,
}

impl AnimationBundle {
    fn spawn(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        root: AnimationRoot,
    ) -> AnimationBundle {
        let texture_handle = asset_server.load("sprites/narf/Idle.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        return AnimationBundle {
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
        };
    }
}
