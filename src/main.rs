#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat3D, RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    LoggerConfig,
};

use crate::{camera_move_system::CameraMoveSystem, core::APP_ROOT, voxel_state::VoxelState};
use std::{fs::OpenOptions, time::Duration};

mod camera_move_system;
mod core;
mod directions;
mod voxel_state;

fn main() -> amethyst::Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(APP_ROOT.join("output.log"))?,
        )
        .apply()
        .unwrap();

    let display_config_path = APP_ROOT.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat3D::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with(
            CameraMoveSystem::default(),
            "move_camera",
            &["input_system", "transform_system"],
        );

    let assets_dir = APP_ROOT.join("assets");
    let mut game = Application::build(assets_dir, VoxelState {})?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
