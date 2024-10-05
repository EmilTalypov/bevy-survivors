use bevy::prelude::*;

use crate::{asset_loader::SpriteAssets, collision::Collider};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map {
            dimensions: IVec2::new(50, 50),
            tile_size: 16,
        })
        .add_systems(PostStartup, add_outer_walls);
    }
}

#[derive(Resource, Debug)]
pub struct Map {
    pub dimensions: IVec2,
    pub tile_size: u32,
}

impl Map {
    pub fn half_extent(&self) -> IVec2 {
        self.dimensions / 2
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Wall;

pub fn grid_to_world(position: IVec2) -> Vec3 {
    (position * 16).extend(-1).as_vec3()
}

#[derive(Bundle, Debug, Default)]
struct WallBundle {
    pub wall: Wall,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: VisibilityBundle,
}

fn add_outer_walls(mut commands: Commands, sprites: Res<SpriteAssets>, map: Res<Map>) {
    let half = map.half_extent();

    // top wall
    commands
        .spawn(WallBundle {
            collider: Collider::with_size_and_offset(
                Vec2::new(map.dimensions.x as f32 * 16., 8.),
                Vec2::new(0., 4.),
            ),
            transform: Transform::from_translation(grid_to_world(IVec2::new(0, half.y))),
            ..default()
        })
        .with_children(|top_wall| {
            for x in (-half.x + 1)..(half.x) {
                top_wall.spawn(SpriteBundle {
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, 0))),
                    texture: sprites.wall.face.clone(),
                    ..default()
                });
            }
        });

    // bottom wall
    commands
        .spawn(WallBundle {
            collider: Collider::with_size_and_offset(
                Vec2::new(map.dimensions.x as f32 * 16., 14.),
                Vec2::new(0., -1.),
            ),
            transform: Transform::from_translation(grid_to_world(IVec2::new(0, -half.y))),
            ..default()
        })
        .with_children(|botttom_wall| {
            botttom_wall.spawn(SpriteBundle {
                transform: Transform::from_translation(grid_to_world(IVec2::new(-half.x, 0))),
                texture: sprites.wall.bottom_left.clone(),
                ..default()
            });

            for x in (-half.x + 1)..(half.x) {
                botttom_wall.spawn(SpriteBundle {
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, 0))),
                    texture: sprites.wall.bottom_mid.clone(),
                    ..default()
                });
            }

            botttom_wall.spawn(SpriteBundle {
                transform: Transform::from_translation(grid_to_world(IVec2::new(half.x, 0))),
                texture: sprites.wall.bottom_right.clone(),
                ..default()
            });
        });

    // left wall
    commands
        .spawn(WallBundle {
            collider: Collider::new(Vec2::new(16., map.dimensions.y as f32 * 16.)),
            transform: Transform::from_translation(grid_to_world(IVec2::new(-half.x, 0))),
            ..default()
        })
        .with_children(|left_wall| {
            for y in (-half.y)..=(half.y) {
                left_wall.spawn(SpriteBundle {
                    transform: Transform::from_translation(grid_to_world(IVec2::new(0, y))),
                    texture: sprites.wall.left.clone(),
                    ..default()
                });
            }
        });

    // right wall
    commands
        .spawn(WallBundle {
            collider: Collider::new(Vec2::new(16., map.dimensions.y as f32 * 16.)),
            transform: Transform::from_translation(grid_to_world(IVec2::new(half.x, 0))),
            ..default()
        })
        .with_children(|left_wall| {
            for y in (-half.y)..=(half.y) {
                left_wall.spawn(SpriteBundle {
                    transform: Transform::from_translation(grid_to_world(IVec2::new(0, y))),
                    texture: sprites.wall.right.clone(),
                    ..default()
                });
            }
        });
}
