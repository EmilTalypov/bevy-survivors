use bevy::prelude::*;

use crate::asset_loader::SpriteAssets;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map {
            dimensions: IVec2::new(100, 100),
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

#[derive(Component, Debug, Clone, Copy)]
pub struct Wall;

pub fn grid_to_world(position: IVec2) -> Vec3 {
    (position * 16).extend(-1).as_vec3()
}

fn add_outer_walls(mut commands: Commands, sprites: Res<SpriteAssets>, map: Res<Map>) {
    let half = map.half_extent();

    // horizontal walls
    for x in (-half.x + 1)..(half.x) {
        for y in [-half.y, half.y] {
            let top_wall = y == half.y;

            let texture = if top_wall {
                &sprites.wall.face
            } else {
                &sprites.wall.bottom_mid
            }
            .clone();

            commands.spawn((
                Wall,
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, y))),
                    ..default()
                },
            ));
        }
    }

    // vertical walls
    for y in (-half.y + 1)..=(half.y) {
        for x in [-half.x, half.x] {
            let left_wall = x == -half.x;

            let texture = if left_wall {
                &sprites.wall.left
            } else {
                &sprites.wall.right
            }
            .clone();

            commands.spawn((
                Wall,
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, y))),
                    ..default()
                },
            ));
        }
    }

    // corners

    for (texture, x, y) in [
        (sprites.wall.bottom_left.clone(), -half.x, -half.y),
        (sprites.wall.bottom_right.clone(), half.x, -half.y),
    ] {
        commands.spawn((
            Wall,
            SpriteBundle {
                texture,
                transform: Transform::from_translation(grid_to_world(IVec2::new(x, y))),
                ..default()
            },
        ));
    }
}
