use amethyst::{
    assets::Loader,
    derive::SystemDesc,
    ecs::prelude::*,
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
    utils::fps_counter::FpsCounter,
};
use log;

pub struct FpsText {
    pub text: Entity,
}

pub fn init_fps_counter(world: &mut World) {
    let transform = UiTransform::new(
        "fps_counter".to_owned(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        10.,
        -10.,
        0.,
        40.,
        25.,
    );

    let font = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let text = UiText::new(
        font,
        "0".to_owned(),
        [1., 1., 1., 1.],
        14.,
        LineMode::Single,
        Anchor::Middle,
    );
    let fps_text_ent = world.create_entity().with(transform).with(text).build();
    world.insert(FpsText { text: fps_text_ent })
}

#[derive(SystemDesc)]
pub struct FpsUiSystem;

impl<'a> System<'a> for FpsUiSystem {
    type SystemData = (
        Read<'a, FpsCounter>,
        ReadExpect<'a, FpsText>,
        WriteStorage<'a, UiText>,
    );

    fn run(&mut self, (counter, text_handle, mut ui_text): Self::SystemData) {
        if let Some(t) = ui_text.get_mut(text_handle.text) {
            t.text = format!("{:.2}", counter.sampled_fps());
        } else {
            log::warn!("No Fps Counter UiText found!");
        }
    }
}
