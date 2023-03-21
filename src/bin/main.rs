use std::{error::Error, path::Path};

use bevy::{
    log::Level,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use voxel_engine_prototype_lib::{
    camera_move_system::{camera_move_system, CameraMoveSensitivity},
    game_config::{GameConfig, GameConfigPlugin},
    voxels::systems::{
        components::{GenerateMapAround, RenderAround},
        materials::Materials,
        VoxelBundle,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    // let mut game_data = DispatcherBuilder::default();
    // game_data
    //     .add_bundle(LoaderBundle)
    //     .add_bundle(VoxelBundle::default())
    let config_path = Path::new("config");

    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: Level::INFO,
            ..default()
        }))
        .add_plugin(GameConfigPlugin::new(GameConfig::from_file_ron(
            config_path.join("game_configs.ron"),
        )?))
        .add_plugin(VoxelBundle)
        .add_startup_system(startup)
        .add_startup_system(add_camera_settings)
        .add_system(camera_move_system)
        .run();

    Ok(())
}

fn add_camera_settings(mut commands: Commands) {
    commands.insert_resource(CameraMoveSensitivity::default());
}

fn startup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.insert_resource(Materials {
        material: debug_material,
    });

    commands.spawn(bevy::pbr::DirectionalLightBundle {
        directional_light: DirectionalLight { ..default() },
        transform: Transform::default().looking_to(Vec3::NEG_Y, Vec3::NEG_Z),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        })
        .insert((RenderAround, GenerateMapAround));
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
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
