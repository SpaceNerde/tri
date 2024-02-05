// Objects in Space
//
// Renders all Objects in Space Like Sun's Planet's and more
// Also renders light

use std::f32::consts::PI;

use bevy::{
    prelude::*,
};

#[derive(Component)]
struct Star;

pub fn spawn_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let yellow_star_material = materials.add(StandardMaterial {
        emissive: Color::YELLOW,
        ..default()
    });

    let sphere = meshes.add(shape::UVSphere::default().into());

    commands.spawn((
        PbrBundle {
            mesh: sphere,
            material: yellow_star_material.clone(),
            transform: Transform::from_xyz(
                0.0,
                0.0,
                0.0,
            )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        Star,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(
            0.,
            0.0,
            0.0
        ),
        ..default()
    });
}