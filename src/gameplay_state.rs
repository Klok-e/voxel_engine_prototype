use crate::{
    game_config::GameConfig,
    ui::{
        chunk_counter::{GeneratedCounterText, RenderedCounterText},
        FpsText,
    },
    voxels::{
        chunk::CHSIZE,
        materials::Materials,
        systems::{
            destroy_on_touch_system::DestroyVoxOnTouch, dirty_around_system::RenderAround,
            generate_map_around_system::GenerateMapAround,
        },
        terrain_generation::ProceduralGenerator,
        world::VoxelWorld,
    },
};
use amethyst::{
    assets::{AssetLoaderSystemData, Handle, Loader},
    core::Transform,
    prelude::*,
    renderer::{
        light::{DirectionalLight, Light},
        loaders,
        mtl::TextureOffset,
        palette::{LinSrgba, Srgb, Srgba},
        resources::AmbientColor,
        Camera, ImageFormat, Material, Texture,
    },
    ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform},
    utils::auto_fov::AutoFov,
};
pub struct GameplayState {}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        init_camera(data.world);

        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);

        data.world.insert(GameConfig {
            chunks_render_per_frame: 10,
            chunks_generate_per_frame: 10,
        });

        data.world
            .insert(VoxelWorld::new(ProceduralGenerator::<CHSIZE>::new(42)));

        data.world
            .insert(AmbientColor(Srgba::new(0.5, 0.5, 0.5, 1.0)));

        let mats = data.world.exec(|(tex, mat)| init_materials(tex, mat));
        data.world.insert(mats);

        // ui
        let font = load_font(data.world);

        init_fps_counter(data.world, font.clone());
        init_chunk_generated_counter(data.world, font.clone());
        init_chunk_rendered_counter(data.world, font.clone());
    }
}

fn load_font(world: &mut World) -> Handle<FontAsset> {
    world
        .read_resource::<Loader>()
        .load("fonts/square.ttf", TtfFormat, (), &world.read_resource())
}

fn init_materials(
    tex_loader: AssetLoaderSystemData<Texture>,
    mat_loader: AssetLoaderSystemData<Material>,
) -> Materials {
    //let albedo = loaders::load_from_srgba(Srgba::new(0.5, 0.7, 0.5, 1.0));
    let emission = loaders::load_from_srgba(Srgba::new(0.0, 0.0, 0.0, 0.0));
    let normal = loaders::load_from_linear_rgba(LinSrgba::new(0.5, 0.5, 1.0, 1.0));
    let metallic_roughness = loaders::load_from_linear_rgba(LinSrgba::new(0.0, 0.5, 0.0, 0.0));
    let ambient_occlusion = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));
    let cavity = loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0));

    //let albedo = tex_loader.load_from_data(albedo.into(), ());
    let albedo = tex_loader.load("blocks/dirt.png", ImageFormat::default(), ());
    let emission = tex_loader.load_from_data(emission.into(), ());
    let normal = tex_loader.load_from_data(normal.into(), ());
    let metallic_roughness = tex_loader.load_from_data(metallic_roughness.into(), ());
    let ambient_occlusion = tex_loader.load_from_data(ambient_occlusion.into(), ());
    let cavity = tex_loader.load_from_data(cavity.into(), ());

    let chunks = mat_loader.load_from_data(
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
    );
    Materials { chunks }
}

fn init_camera(world: &mut World) {
    let mut light = DirectionalLight::default();
    light.color = Srgb::new(1., 1., 1.);
    world
        .create_entity()
        .with(Light::Directional(light))
        .build();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 2.);

    world
        .create_entity()
        .with(Camera::standard_3d(10., 10.))
        .with(AutoFov::new())
        .with(transform)
        .with(RenderAround::new(1))
        .with(GenerateMapAround::new(10))
        .with(DestroyVoxOnTouch)
        .build();
}

fn init_fps_counter(world: &mut World, font: Handle<FontAsset>) {
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
        font,
        "".to_owned(),
        [1., 1., 1., 1.],
        14.,
        LineMode::Single,
        Anchor::Middle,
    );
    let fps_text_ent = world.create_entity().with(transform).with(text).build();
    world.insert(FpsText { text: fps_text_ent });
}

fn init_chunk_generated_counter(world: &mut World, font: Handle<FontAsset>) {
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
        font,
        "".to_owned(),
        [1., 1., 1., 1.],
        14.,
        LineMode::Single,
        Anchor::Middle,
    );
    let entity = world.create_entity().with(transform).with(text).build();
    world.insert(GeneratedCounterText { entity });
}

fn init_chunk_rendered_counter(world: &mut World, font: Handle<FontAsset>) {
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
        font,
        "".to_owned(),
        [1., 1., 1., 1.],
        14.,
        LineMode::Single,
        Anchor::Middle,
    );
    let entity = world.create_entity().with(transform).with(text).build();
    world.insert(RenderedCounterText { entity });
}
