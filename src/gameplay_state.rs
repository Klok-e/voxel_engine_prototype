use crate::game_config::GameConfig;
use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{
        light::{DirectionalLight, Light},
        palette::{Srgb, Srgba},
        resources::AmbientColor,
        Camera,
    },
    ui::{Anchor, LineMode, UiText, UiTransform},
    utils::auto_fov::AutoFov,
};
pub struct GameplayState {}

impl SimpleState for GameplayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        init_camera(&mut data);

        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);

        data.resources.insert(GameConfig {
            chunks_render_per_frame: 10,
            chunks_generate_per_frame: 10,
        });

        // data.world
        //     .insert(VoxelWorld::new(ProceduralGenerator::<CHSIZE>::new(42)));

        data.resources
            .insert(AmbientColor(Srgba::new(0.5, 0.5, 0.5, 1.0)));

        // let mats = data.world.exec(|(tex, mat)| init_materials(tex, mat));
        // data.world.insert(mats);

        // ui
        init_fps_counter(&mut data);
        init_chunk_generated_counter(&mut data);
        init_chunk_rendered_counter(&mut data);
    }
}

// fn init_materials(
//     tex_loader: AssetLoaderSystemData<Texture>,
//     mat_loader: AssetLoaderSystemData<Material>,
// ) -> Materials {
//     //let albedo = loaders::load_from_srgba(Srgba::new(0.5, 0.7, 0.5, 1.0));
//     let emission = loaders::load_from_srgba(Srgba::new(0.0, 0.0, 0.0, 0.0));
//     let normal = loaders::load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 1.0, 1.0));
//     let metallic_roughness = loaders::load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0));
//     let ambient_occlusion = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));
//     let cavity = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));

//     //let albedo = tex_loader.load_from_data(albedo.into(), ());
//     let albedo = tex_loader.load("blocks/dirt.png", ImageFormat::default(), ());
//     let emission = tex_loader.load_from_data(emission.into(), ());
//     let normal = tex_loader.load_from_data(normal.into(), ());
//     let metallic_roughness = tex_loader.load_from_data(metallic_roughness.into(), ());
//     let ambient_occlusion = tex_loader.load_from_data(ambient_occlusion.into(), ());
//     let cavity = tex_loader.load_from_data(cavity.into(), ());

//     let chunks = mat_loader.load_from_data(
//         Material {
//             alpha_cutoff: 0.01,
//             albedo,
//             emission,
//             normal,
//             metallic_roughness,
//             ambient_occlusion,
//             cavity,
//             uv_offset: TextureOffset::default(),
//         },
//         (),
//     );
//     Materials { chunks }
// }

fn init_camera(state: &mut StateData<GameData>) {
    let mut light = DirectionalLight::default();
    light.color = Srgb::new(1., 1., 1.);
    state.world.push((Light::Directional(light),));

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 2.);

    state.world
        .push((Camera::standard_3d(10., 10.),AutoFov::new(),transform))
        // .with(RenderAround::new(1))
        // .with(GenerateMapAround::new(10))
        // .with(DestroyVoxOnTouch)
        ;
}

fn init_fps_counter(state: &mut StateData<GameData>) {
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

    let text = UiText::new(
        None,
        "".to_owned(),
        [1., 1., 1., 1.],
        14.,
        LineMode::Single,
        Anchor::Middle,
    );
    let fps_text_ent = state.world.push((transform, text));
    // state.resources.insert(FpsText { text: fps_text_ent });
}

fn init_chunk_generated_counter(state: &mut StateData<GameData>) {
    let transform = UiTransform::new(
        "fps_counter".to_owned(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        100.,
        -10.,
        0.,
        100.,
        25.,
    );

    let text = UiText::new(
        None,
        "".to_owned(),
        [1., 1., 1., 1.],
        14.,
        LineMode::Single,
        Anchor::Middle,
    );
    let entity = state.world.push((transform, text));
    // world.insert(GeneratedCounterText { entity });
}

fn init_chunk_rendered_counter(state: &mut StateData<GameData>) {
    let transform = UiTransform::new(
        "fps_counter".to_owned(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        200.,
        -10.,
        0.,
        100.,
        25.,
    );

    let text = UiText::new(
        None,
        "".to_owned(),
        [1., 1., 1., 1.],
        14.,
        LineMode::Single,
        Anchor::Middle,
    );
    let entity = state.world.push((transform, text));
    // world.insert(RenderedCounterText { entity });
}
