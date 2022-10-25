use bevy::prelude::*;
use bevy::time::FixedTimestep;

mod fllow;

use crate::{display::cell::Cell, CELL_SIZE, TIME_STEP};

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let test_message = Message {
            correlation: 0.7,
            readability: 0.8,
            benefits: 0.4,
            reliability: 0.2,
            face: Color::BLACK,
        };

        app.insert_resource(test_message)
            .add_event::<fllow::Deliver>()
            .add_system(cursor_change_cell_color)
            .add_system_set(
                SystemSet::new()
                    .label(fllow::FllowLable::Receive)
                    .with_system(fllow::receive_message),
            )
            .add_system_set(
                SystemSet::new()
                    .label(fllow::FllowLable::Send)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP * 5.))
                    .with_system(fllow::send_message),
            );
    }
}

/// 在人群之间传播的信息
#[derive(Debug, Clone, Component)]
pub struct Message {
    /// 相关性
    pub correlation: f32,
    /// 易读性
    pub readability: f32,
    /// 传播收益
    pub benefits: f32,
    /// 可靠性
    pub reliability: f32,
    pub face: Color,
}

impl Message {
    pub fn accept_ratio(&self) -> f32 {
        (self.correlation + self.readability + self.reliability) / 3.
    }

    pub fn spread_benefits(&self) -> f32 {
        self.benefits
    }
}

fn cursor_change_cell_color(
    windows: Res<Windows>,
    message: Res<Message>,
    buttons: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut cells: Query<(&mut Sprite, &mut Cell, &GlobalTransform)>,
) {
    if buttons.pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();

        let wnd = windows.get_primary().unwrap();

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            cells
                .iter_mut()
                .filter(|(_, _, &trans)| {
                    let cell_pos = trans.translation().truncate();
                    cell_pos.distance(world_pos) <= CELL_SIZE / 2.
                })
                .for_each(|(mut sprite, mut cell, _)| {
                    sprite.color = message.face;
                    cell.acc_message = Some(message.to_owned());
                });
        }
    }
}
