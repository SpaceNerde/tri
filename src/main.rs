mod ois;
mod pis;

use bevy::{
    prelude::*,
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
};
use crate::ois::spawn_star;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "TRI".to_string(),
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            // The canvas size is constrained in index.html and build/web/styles.css
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }).set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, spawn_star))
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // spawn main scene camera
    commands.spawn((Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
       tonemapping: Tonemapping::TonyMcMapface,
       transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, BloomSettings::default(),
    ));
}