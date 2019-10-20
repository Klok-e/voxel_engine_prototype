use amethyst::prelude::*;
use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    LoggerConfig,
};

mod camera_move_system;
mod voxel_state;

use crate::camera_move_system::CameraMoveSystem;
use voxel_state::VoxelState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderShaded3D::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with(
            CameraMoveSystem::default(),
            "move_camera",
            &["input_system", "transform_system"],
        );

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, VoxelState {}, game_data)?;
    game.run();

    Ok(())
}
