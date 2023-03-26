use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{default, AssetServer, Color, Commands, Plugin, Res, TextBundle},
    text::{TextAlignment, TextSection, TextStyle},
    ui::{PositionType, Style, UiRect, Val},
};

use super::{
    chunk_counter::{chunk_counter_ui_system, ChunkCountersText},
    fps_counter::{fps_ui_system, FpsText},
};

pub struct DebugUiBundle;

impl Plugin for DebugUiBundle {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_system(chunk_counter_ui_system);
        app.add_system(fps_ui_system);

        app.add_startup_system(startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ])
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Chunks: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                "; Dirty: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::GOLD,
            }),
            TextSection::new(
                "; Rendered: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font,
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ])
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(35.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        ChunkCountersText,
    ));
}
