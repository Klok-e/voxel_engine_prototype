use bevy::prelude::Component;

#[derive(Component)]
pub struct GenerateMapAround;

#[derive(Component)]
pub struct RenderedTag;

#[derive(Component)]
pub struct RenderAround;

#[derive(Debug, Default, Component)]
pub struct DestroyVoxOnTouch;
