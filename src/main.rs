//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::{
    log::LogPlugin,
    pbr::wireframe::{NoWireframe, Wireframe, WireframePlugin},
    prelude::*,
    render::{
        mesh::shape::Plane,
        render_resource::{AsBindGroup, ShaderRef},
    },
};
use bevy_obj::ObjPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

mod grid;
mod solver;
mod visualizers;

use grid::CartesianGrid;
use solver::Solver;

const DEFAULT_MESH: &str = "cube.obj";

fn main() {
    App::new()
        // Default Functionality Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(ObjPlugin)
        .add_plugins(MaterialPlugin::<ShaderTestMaterial>::default())
        // Camera Plugins
        .add_plugins(PanOrbitCameraPlugin)
        // Other Plugions
        .add_plugins(Solver)
        .add_systems(Startup, setup)
        .insert_resource(CartesianGrid::new(4.0, 4.0, 4.0, 1, 10, 10))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut test_materials: ResMut<Assets<ShaderTestMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    grid: Res<CartesianGrid>,
) {
    // // mesh (default cube)
    // let mesh_handle = asset_server.load(DEFAULT_MESH);
    // commands.spawn(PbrBundle {
    //     mesh: mesh_handle,
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     material: materials.add(Color::rgb_u8(124, 144, 255).into()),
    //     ..default()
    // });

    // slicing plane
    dbg!(grid);
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(Plane { size: 1.0, subdivisions: 1 })),
    //     material: test_materials.add(ShaderTestMaterial {
    //         color: Color::rgba(0.0, 0.0, 1.0, 1.0),
    //     }),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..Default::default()
    // });
    commands.spawn(MaterialMeshBundle::<ShaderTestMaterial> {
        mesh: meshes.add(Mesh::from(Plane {
            size: 1.0,
            subdivisions: 1,
        })),
        material: test_materials.add(ShaderTestMaterial {
            color: Color::WHITE,
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            // transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            transform: Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ShaderTestMaterial {
    #[uniform(100)]
    color: Color,
}
impl Material for ShaderTestMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/pressure_field.wgsl".into()
    }
}
