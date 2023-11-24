use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "π Estimator".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn floor
    let floor_mesh = meshes.add(shape::Quad::new(Vec2::new(99999., 5.)).into());
    commands.spawn(MaterialMesh2dBundle {
        mesh: floor_mesh.into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform {
            translation: Vec3::new(0., -200., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}
