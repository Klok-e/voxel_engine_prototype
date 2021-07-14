use amethyst::ecs::{query::Query, SystemBuilder};
use amethyst::{
    ecs::{Entity, IntoQuery, Runnable, SubWorld},
    ui::UiText,
    utils::fps_counter::FpsCounter,
};
use log;

pub struct FpsText {
    pub text: Entity,
}

pub fn fps_ui_system() -> impl Runnable {
    SystemBuilder::new("fps_ui_system")
        .read_resource::<FpsCounter>()
        .read_resource::<FpsText>()
        .with_query(<(&mut UiText,)>::query())
        .build(move |_, world, resources, query| fps_ui(world, &resources.0, &resources.1, query))
}

fn fps_ui(
    world: &mut SubWorld,
    counter: &FpsCounter,
    text_handle: &FpsText,
    q: &mut Query<(&mut UiText,)>,
) {
    match q.get_mut(world, text_handle.text) {
        Ok(t) => {
            t.0.text = format!("{:.2}", counter.sampled_fps());
        }
        Err(e) => {
            log::warn!("No Fps Counter UiText found! {}", e);
        }
    }
}
