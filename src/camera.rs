use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

const CAMERA_SCALE: f32 = 0.75;

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = CAMERA_SCALE;
    commands.spawn(camera);
}
