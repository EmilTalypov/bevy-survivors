use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub ghost: Handle<Image>,
    pub dagger: Handle<Image>,
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
        dagger: asset_server.load("tile_0103.png"),
    }
}
