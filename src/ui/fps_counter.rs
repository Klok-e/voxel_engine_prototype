use amethyst::{
    ecs::{system, Entity, IntoQuery, SubWorld},
    ui::UiText,
    utils::fps_counter::FpsCounter,
};
use log;

pub struct FpsText {
    pub text: Entity,
}

#[system]
#[write_component(UiText)]
pub fn fps_ui_system(
    world: &mut SubWorld,
    #[resource] counter: &FpsCounter,
    #[resource] text_handle: &FpsText,
) {
    let mut q = <(&mut UiText,)>::query();

    match q.get_mut(world, text_handle.text) {
        Ok(t) => {
            t.0.text = format!("{:.2}", counter.sampled_fps());
        }
        Err(e) => {
            log::warn!("No Fps Counter UiText found! {}", e);
        }
    }
}
