use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{player::Player, schedule::InGame};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follows_player.in_set(InGame::EntityUpdate));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_follows_player(
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<(&mut Transform, &OrthographicProjection), Without<Player>>,
    levels_q: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    level_selection: Res<LevelSelection>,
    project: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };

    for (level_transform, level_iid) in &levels_q {
        let project = project_assets.get(project.single()).expect("No project!");

        let level = project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("No level!");

        if level_selection.is_match(&LevelIndices::default(), level) {
            let (mut camera_transform, projection) = camera_q.get_single_mut().expect("No Camera!");

            let top_right = Vec3::new(level.px_wid as f32, level.px_hei as f32, 0.);
            let bottom_left = Vec3::ZERO;

            let min_camera_position =
                bottom_left - projection.area.min.extend(0.) + level_transform.translation;
            let max_camera_position =
                top_right - projection.area.max.extend(0.) + level_transform.translation;

            camera_transform.translation = player_transform
                .translation
                .clamp(min_camera_position, max_camera_position);
        }
    }
}
