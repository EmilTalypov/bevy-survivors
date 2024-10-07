use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod asset_loader;
mod camera;
mod collision;
mod combat;
pub mod ghost;
mod health;
mod levels;
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
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Survivors".to_string(),
                        resolution: (800., 600.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LdtkPlugin)
        // my plugins
        .add_plugins(schedule::SchedulePlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(levels::LevelsPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(ghost::GhostPlugin)
        .add_plugins(health::HealthPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}
