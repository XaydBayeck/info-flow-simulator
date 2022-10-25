use bevy::prelude::*;

use crate::DEFAULT_WORLD_SIZE;

pub mod cell;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DEFAULT_WORLD_SIZE)
            .add_startup_system(setup_camera)
            .add_startup_system(cell::setup_map);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
