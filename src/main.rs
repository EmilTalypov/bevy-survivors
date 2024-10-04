use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use ghost::GhostPlugin;
use health::HealthPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;

mod asset_loader;
mod camera;
mod collision;
mod ghost;
mod health;
mod movement;
pub mod player;
pub mod schedule;

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::srgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::srgb(1., 1., 1.),
            brightness: 0.95,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Survivors".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        // my plugins
        .add_plugins(SchedulePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(GhostPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
