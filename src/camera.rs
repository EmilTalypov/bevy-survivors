use bevy::prelude::*;

use crate::{map::grid_to_world, map::Map, player::Player, schedule::InGame};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follows_player.in_set(InGame::EntityUpdate));
    }
}

const CAMERA_SCALE: f32 = 0.75;

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = CAMERA_SCALE;
    commands.spawn(camera);
}

fn camera_follows_player(
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<(&mut Transform, &OrthographicProjection), Without<Player>>,
    map: Res<Map>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };

    let (mut camera_transform, projection) = camera_q.get_single_mut().expect("No Camera!");

    let bottom_left = grid_to_world(-(map.dimensions / 2));
    let top_right = grid_to_world(map.dimensions / 2);

    let min_camera_position = bottom_left - projection.area.min.extend(0.);
    let max_camera_position = top_right - projection.area.max.extend(0.);

    camera_transform.translation = player_transform
        .translation
        .clamp(min_camera_position, max_camera_position);
}
