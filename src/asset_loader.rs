use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub ghost: Handle<Image>,
    pub knight: Handle<Image>,
    pub dagger: Handle<Image>,
    pub wall: WallSprites,
}

#[derive(Debug, Default)]
pub struct WallSprites {
    pub bottom_mid: Handle<Image>,
    pub bottom_left: Handle<Image>,
    pub bottom_right: Handle<Image>,
    pub face: Handle<Image>,
    pub left: Handle<Image>,
    pub right: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut sprites: ResMut<SpriteAssets>, asset_server: Res<AssetServer>) {
    *sprites = SpriteAssets {
        ghost: asset_server.load("tile_0121.png"),
        knight: asset_server.load("tile_0097.png"),
        dagger: asset_server.load("tile_0103.png"),
        wall: WallSprites {
            face: asset_server.load("tile_0040.png"),
            bottom_left: asset_server.load("tile_0025.png"),
            bottom_mid: asset_server.load("tile_0026.png"),
            bottom_right: asset_server.load("tile_0027.png"),
            left: asset_server.load("tile_0013.png"),
            right: asset_server.load("tile_0015.png"),
        },
    }
}
