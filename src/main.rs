use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};
use bevy_life::SimulationBatch;

mod cell;
mod consts;
mod display;
mod message;

pub use consts::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(display::DisplayPlugin)
        .add_plugin(message::MessagePlugin)
        .add_plugin(cell::CellRulePlugin::new(TIME_STEP))
        //.register_inspectable::<message::Message>()
        //.register_inspectable::<cell::Individual>()
        .insert_resource(SimulationBatch::default());

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
