use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{auto_fov::AutoFovSystem, fps_counter::FpsCounterBundle},
    LogLevelFilter, Logger, LoggerConfig,
};
use std::time::Duration;
use voxel_engine_prototype_lib::{
    camera_move_system::CameraMoveSystem,
    core::APP_ROOT,
    destroy_on_touch_system::DestroyOnTouchSystem,
    gameplay_state::GameplayState,
    ui::FpsUiSystem,
    voxels::{
        dirty_around_system::DirtyAroundSystem,
        generate_map_around_system::GenerateMapAroundSystem, ChunkRenderSystem,
    },
    world_change_apply_system::WorldApplyChangesSystem,
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
        .with(DestroyOnTouchSystem, "destroy_on_touch_system", &[])
        .with(
            WorldApplyChangesSystem,
            "world_apply_changes_system",
            &["destroy_on_touch_system"],
        )
        .with(DirtyAroundSystem, "dirty_around_system", &[])
        .with(
            ChunkRenderSystem,
            "chunks_system",
            &["world_apply_changes_system", "dirty_around_system"],
        )
        .with(GenerateMapAroundSystem, "generate_map_around_system", &[]);

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
