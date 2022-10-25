use bevy::prelude::*;

mod display;
mod message;
mod consts;

pub use consts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(display::DisplayPlugin)
        .add_plugin(message::MessagePlugin)
        .run();
}
