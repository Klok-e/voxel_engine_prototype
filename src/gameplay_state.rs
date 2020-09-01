use crate::{
    camera_move_system::init_camera,
    game_config::GameConfig,
    ui::init_fps_counter,
    voxels::{create_cube, materials::Materials},
};
use amethyst::{
    assets::AssetLoaderSystemData,
    core::Transform,
    prelude::*,
    renderer::{
        loaders,
        palette::{LinSrgba, Srgba},
        Material, Texture, resources::AmbientColor,
    },
    renderer::{mtl::TextureOffset, ImageFormat},
};
pub struct GameplayState {}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        init_camera(data.world);
        init_fps_counter(data.world);
        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);
        create_cube(data.world, transform);

        data.world.insert(GameConfig {
            chunks_render_per_frame: 10,
            chunks_generate_per_frame:10,
        });

        data.world
            .insert(AmbientColor(Srgba::new(0.5, 0.5, 0.5, 1.0)));

        let mats = data.world.exec(|(tex, mat)| init_materials(tex, mat));
        data.world.insert(mats);
    }
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
