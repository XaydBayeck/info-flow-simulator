use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
};

use crate::{message::Message, WorldSize, BORDER, CELL_SIZE};

#[derive(Component, Default)]
pub struct Cell {
    pub acc_message: Option<Message>,
}

#[derive(Component)]
pub struct Cells;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate(pub usize, pub usize);

#[derive(Bundle)]
pub struct CellBundle {
    cell: Cell,
    coor: Coordinate,
    #[bundle]
    sprite: SpriteBundle,
}

impl CellBundle {
    pub fn new(size: f32, col: usize, row: usize) -> Self {
        Self {
            cell: Cell::default(),
            coor: Coordinate(col, row),
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    size * col as f32,
                    size * row as f32,
                    0.,
                )),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(CELL_SIZE - BORDER)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn setup_map(mut commands: Commands, world_size: Res<WorldSize>) {
    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
            -(CELL_SIZE * world_size.col() as f32) / 2.,
            -(CELL_SIZE * world_size.row() as f32) / 2.,
            0.,
        )))
        .insert(Cells)
        .with_children(|parent| {
            for row in 0..=world_size.row() {
                for col in 0..=world_size.col() {
                    parent.spawn_bundle(CellBundle::new(CELL_SIZE, col, row));
                }
            }
        });
}
