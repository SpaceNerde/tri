// All the other stuff like make camera rotate and idk maybe other stuff later

use bevy::prelude::*;

pub fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>
) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y((time.delta_seconds() / 2.)));
}