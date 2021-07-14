use amethyst::{
    assets::LoaderBundle,
    controls::FlyControlBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::DispatcherBuilder,
    input::InputBundle,
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
use log::info;
use std::{str::FromStr, time::Duration};
use voxel_engine_prototype_lib::{
    core::APP_ROOT,
    game_config::{ConfigsBundle, GameConfig},
    gameplay_state::GameplayState,
    ui::{chunk_counter::chunk_counter_ui_system, fps_counter::fps_ui_system},
    voxels::systems::VoxelBundle,
};

fn main() -> amethyst::Result<()> {
    Logger::from_config_formatter(
        LoggerConfig {
            level_filter: LogLevelFilter::from_str(
                &std::env::var("DEBUG").unwrap_or("warn".to_string()),
            )?,
            log_file: Some("./output.log".parse()?),
            module_levels: vec![
                ("amethyst".to_string(), LogLevelFilter::Info),
                ("amethyst_assets".to_string(), LogLevelFilter::Info),
                ("distill_daemon".to_string(), LogLevelFilter::Info),
                ("winit".to_string(), LogLevelFilter::Info),
            ],
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
    info!("App root: {}", APP_ROOT.to_string_lossy());

    let mut game_data = DispatcherBuilder::default();
    game_data
        .add_bundle(LoaderBundle)
        .add_bundle(InputBundle::new().with_bindings_from_file(config_path.join("bindings.ron"))?)
        .add_bundle(ConfigsBundle::new(GameConfig::from_file_ron(
            config_path.join("game_configs.ron"),
        )?))
        .add_bundle(TransformBundle::default())
        .add_bundle(FpsCounterBundle::default())
        .add_bundle(UiBundle::<u32>::new())
        .add_bundle(
            FlyControlBundle::new(
                Some("move_x".into()),
                Some("move_y".into()),
                Some("move_z".into()),
            )
            .with_sensitivity(0.1, 0.1)
            .with_speed(2.),
        )
        .add_bundle(VoxelBundle::default())
        .add_system(AutoFovSystem)
        .add_system(|| chunk_counter_ui_system())
        .add_system(|| fps_ui_system())
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
        );

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
