// Objects in Space
//
// Renders all Objects in Space Like Sun's Planet's and more
// Also renders light

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
};

#[derive(Component)]
struct Star;

pub fn spawn_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // credit to: https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_shapes.rs
    let debug_material = materials.add(StandardMaterial {
        //base_color_texture: Some(images.add(uv_debug_texture())),
        emissive: Color::YELLOW,
        ..default()
    });

    let sphere = meshes.add(shape::UVSphere::default().into());

    commands.spawn((
        PbrBundle {
            mesh: sphere,
            material: debug_material.clone(),
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

// credit to: https://github.com/bevyengine/bevy/blob/latest/examples/3d/3d_shapes.rs
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}