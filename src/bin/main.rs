use amethyst::{
    assets::LoaderBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::DispatcherBuilder,
    input::{Bindings, InputBundle},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderShaded3D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{auto_fov::AutoFovSystem, fps_counter::FpsCounterBundle},
    LogLevelFilter, Logger, LoggerConfig,
};
use std::time::Duration;
use voxel_engine_prototype_lib::{
    camera_move_system::ControlsBundle,
    core::APP_ROOT,
    gameplay_state::GameplayState,
    ui::{chunk_counter::chunk_counter_ui_system, fps_counter::fps_ui_system},
    voxels::systems::VoxelBundle,
};

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

    let config_path = APP_ROOT.join("config");
    let display_config_path = config_path.join("display.ron");
    dbg!(&*APP_ROOT);
    let mut game_data = DispatcherBuilder::default();
    game_data
        .add_bundle(LoaderBundle)
        .add_bundle(InputBundle::new().with_bindings_from_file(config_path.join("bindings.ron"))?)
        .add_bundle(TransformBundle::default())
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor {
                        float32: [0.34, 0.36, 0.52, 1.0],
                    }),
                )
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default()),
        )
        .add_bundle(FpsCounterBundle::default())
        .add_bundle(UiBundle::<u32>::new())
        .add_bundle(ControlsBundle::default())
        .add_bundle(VoxelBundle::default())
        .add_system(AutoFovSystem)
        .add_system(|| chunk_counter_ui_system())
        .add_system(|| fps_ui_system());

    let assets_dir = APP_ROOT.join("assets");
    let game = Application::build(assets_dir, GameplayState {})?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(1)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
