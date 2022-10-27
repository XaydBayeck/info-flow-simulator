use bevy::prelude::*;
use bevy_life::MooreCell2d;

use crate::cell::Individual;
use crate::{WorldSize, CELL_SIZE, DEFAULT_WORLD_SIZE, BORDER};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WindowDescriptor {
            title: "rumor flow simulator".to_string(),
            width: 1080.0,
            height: 900.0,
            ..Default::default()
        })
        .insert_resource(DEFAULT_WORLD_SIZE)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn setup_map(mut commands: Commands, world_size: Res<WorldSize>) {
    let (cols, rows) = (world_size.col(), world_size.row());
    let color = Color::WHITE;

    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
            -(cols as f32 * CELL_SIZE) / 2.,
            -(rows as f32 * CELL_SIZE) / 2.,
            0.,
        )))
        .with_children(|parent| {
            for col in 0..=cols {
                for row in 0..=rows {
                    // TODO: Add Cell State Component
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(CELL_SIZE - BORDER)),
                                color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                CELL_SIZE * col as f32,
                                CELL_SIZE * row as f32,
                                0.,
                            ),
                            visibility: Visibility { is_visible: true },
                            ..Default::default()
                        })
                        .insert(MooreCell2d::new(IVec2::new(col, row)))
                        .insert(Individual::default())
                        .insert(Name::new(format!("Cell {row}x{col}")));
                }
            }
        });
}
