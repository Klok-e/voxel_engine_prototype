#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat3D, RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    utils::fps_counter::FpsCounterBundle,
    LogLevelFilter, Logger, LoggerConfig, StdoutLog,
};

use crate::voxels::ChunkRenderSystem;
use crate::{
    camera_move_system::CameraMoveSystem, core::APP_ROOT, gameplay_state::GameplayState,
    ui::FpsUiSystem,
};
use amethyst::utils::auto_fov::AutoFovSystem;
use std::{fs::OpenOptions, time::Duration};

mod camera_move_system;
mod core;
mod directions;
mod gameplay_state;
mod ui;
mod voxels;

fn main() -> amethyst::Result<()> {
    Logger::from_config_formatter(
        LoggerConfig {
            level_filter: LogLevelFilter::Warn,
            log_file: Some("./output.log".parse()?),
            ..LoggerConfig::default()
        },
        |out, message, record| {
            out.finish(format_args!(
                "[{time}][{level}][{target}] {message}",
                time = chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        },
    )
    .start();

    let display_config_path = APP_ROOT.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(AutoFovSystem::new(), "auto_fov_system", &[])
        .with(FpsUiSystem, "show_fps_system", &["fps_counter_system"])
        .with(
            CameraMoveSystem::default(),
            "move_camera_system",
            &["input_system", "transform_system"],
        )
        .with(ChunkRenderSystem, "chunks_system", &[]);

    let assets_dir = APP_ROOT.join("assets");
    let mut game = Application::build(assets_dir, GameplayState {})?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(1)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
