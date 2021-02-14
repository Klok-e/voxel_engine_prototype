use crate::{camera_move_system::CameraMoveSensitivity, ui::{chunk_counter::{DirtyCounterText, GeneratedCounterText, RenderedCounterText}, fps_counter::FpsText}, voxels::{
        chunk::CHSIZE,
        materials::Materials,
        systems::{
            destroy_on_touch_system::DestroyVoxOnTouch, dirty_around_system::RenderAround,
            generate_map_around_system::GenerateMapAround,
        },
        terrain_generation::ProceduralGenerator,
        world::VoxelWorld,
    }};
use amethyst::{
    assets::{DefaultLoader, Loader, ProcessingQueue},
    controls::{FlyControl, HideCursor},
    core::Transform,
    prelude::*,
    renderer::{
        light::{DirectionalLight, Light},
        loaders,
        mtl::TextureOffset,
        palette::{LinSrgba, Srgb, Srgba},
        resources::AmbientColor,
        types::TextureData,
        Camera, Material,
    },
    ui::{Anchor, LineMode, UiText, UiTransform},
    utils::auto_fov::AutoFov,
};

#[derive(Debug)]
pub struct GameplayState {}

impl SimpleState for GameplayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        init_camera(&mut data);

        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);

        data.resources
            .insert(VoxelWorld::new(ProceduralGenerator::<CHSIZE>::new(42)));

        data.resources
            .insert(AmbientColor(Srgba::new(0.5, 0.5, 0.5, 1.0)));
        let mats;
        {
            let loader = data.resources.get::<DefaultLoader>().unwrap();
            let tex_queue = data
                .resources
                .get::<ProcessingQueue<TextureData>>()
                .unwrap();
            let mat_queue = data.resources.get::<ProcessingQueue<Material>>().unwrap();
            mats = init_materials(&*loader, &*tex_queue, &*mat_queue);
        }
        data.resources.insert(mats);

        // hide cursor
        {
            let mut hide = data.resources.get_mut::<HideCursor>().unwrap();
            hide.hide = true;
        }

        // ui
        init_fps_counter(&mut data);
        init_chunk_generated_counter(&mut data);
        init_chunk_rendered_counter(&mut data);
        init_chunk_dirty_counter(&mut data);
    }
}

fn init_materials(
    loader: &DefaultLoader,
    tex_queue: &ProcessingQueue<TextureData>,
    mat_queue: &ProcessingQueue<Material>,
) -> Materials {
    //let albedo = loaders::load_from_srgba(Srgba::new(0.5, 0.7, 0.5, 1.0));
    let emission = loaders::load_from_srgba(Srgba::new(0.0, 0.0, 0.0, 0.0));
    let normal = loaders::load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 1.0, 1.0));
    let metallic_roughness = loaders::load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0));
    let ambient_occlusion = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));
    let cavity = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));

    //let albedo = tex_loader.load_from_data(albedo.into(), ());
    let albedo = loader.load("blocks/dirt.png");
    let emission = loader.load_from_data(emission.into(), (), tex_queue);
    let normal = loader.load_from_data(normal.into(), (), tex_queue);
    let metallic_roughness = loader.load_from_data(metallic_roughness.into(), (), tex_queue);
    let ambient_occlusion = loader.load_from_data(ambient_occlusion.into(), (), tex_queue);
    let cavity = loader.load_from_data(cavity.into(), (), tex_queue);

    let chunks = loader.load_from_data(
        Material {
            alpha_cutoff: 0.01,
            albedo,
            emission,
            normal,
            metallic_roughness,
            ambient_occlusion,
            cavity,
            uv_offset: TextureOffset::default(),
        },
        (),
        mat_queue,
    );
    Materials { chunks }
}

fn init_camera(state: &mut StateData<GameData>) {
    state.resources.insert(CameraMoveSensitivity::default());

    let mut light = DirectionalLight::default();
    light.color = Srgb::new(1., 1., 1.);
    state.world.push((Light::Directional(light),));

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 2.);

    state.world.push((
        Camera::standard_3d(10., 10.),
        FlyControl,
        AutoFov::new(),
        transform,
        RenderAround,
        GenerateMapAround,
        DestroyVoxOnTouch,
    ));
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
    state.resources.insert(FpsText { text: fps_text_ent });
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
    state.resources.insert(GeneratedCounterText { entity });
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
    state.resources.insert(RenderedCounterText { entity });
}

fn init_chunk_dirty_counter(state: &mut StateData<GameData>) {
    let transform = UiTransform::new(
        "fps_counter".to_owned(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        300.,
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
    state.resources.insert(DirtyCounterText { entity });
}
