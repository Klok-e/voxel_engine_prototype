use amethyst::{
    ecs::{system, systems, Entity, SubWorld},
    ui::UiText,
};
use log;

pub struct GeneratedCounterText {
    pub entity: Entity,
}

pub struct RenderedCounterText {
    pub entity: Entity,
}

#[system]
//#[read_component(ChunkPosition)]
#[write_component(UiText)]
pub fn chunk_counter_ui_system(
    world: &mut SubWorld,
    // #[resource] voxel_world: &VoxelWorldProcedural,
    #[resource] generated_text: &GeneratedCounterText,
    #[resource] rendered_text: &RenderedCounterText,
) {
    // let mut q = <(&mut UiText, &ChunkPosition)>::query();
    // if let Some(t) = ui_text.get_mut(generated_text.entity) {
    //                 t.text = format!("ch gen: {}", voxel_world.chunks().len());
    //             } else {
    //                 log::warn!("No GeneratedCounterText UiText found!");
    //             }

    //             if let Some(t) = ui_text.get_mut(rendered_text.entity) {
    //                 t.text = format!("ch rend: {}", ch_positions.join().count());
    //             } else {
    //                 log::warn!("No RenderedCounterText UiText found!");
    //             }
}

// #[derive(SystemDesc)]
// pub struct ChunkCounterUiSystem;

// impl<'a> System<'a> for ChunkCounterUiSystem {
//     type SystemData = (
//         ReadExpect<'a, VoxelWorldProcedural>,
//         ReadExpect<'a, GeneratedCounterText>,
//         ReadExpect<'a, RenderedCounterText>,
//         WriteStorage<'a, UiText>,
//         ReadStorage<'a, ChunkPosition>,
//     );

//     fn run(
//         &mut self,
//         (voxel_world, generated_text, rendered_text, mut ui_text, ch_positions): Self::SystemData,
//     ) {
//         if let Some(t) = ui_text.get_mut(generated_text.entity) {
//             t.text = format!("ch gen: {}", voxel_world.chunks().len());
//         } else {
//             log::warn!("No GeneratedCounterText UiText found!");
//         }

//         if let Some(t) = ui_text.get_mut(rendered_text.entity) {
//             t.text = format!("ch rend: {}", ch_positions.join().count());
//         } else {
//             log::warn!("No RenderedCounterText UiText found!");
//         }
//     }
// }
