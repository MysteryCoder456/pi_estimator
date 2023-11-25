use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::{collide_aabb, MaterialMesh2dBundle, Mesh2dHandle},
};

const CUBE_SCALE: f32 = 20.;
const WALL_POSITION: f32 = -452.;

#[derive(Component)]
struct Cube {
    mass: f32,
    velocity: f32,
}

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
        .add_systems(
            FixedUpdate,
            (
                cube_velocity_system,
                cube_collision_system,
                wall_collision_system,
            ),
        )
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

    // Prepare meshes
    let floor_mesh: Mesh2dHandle = meshes
        .add(shape::Quad::new(Vec2::new(99999., 4.)).into())
        .into();
    let cube_mesh: Mesh2dHandle = meshes
        .add(shape::Quad::new(Vec2::new(1., 1.)).into())
        .into();

    // Prepare materials
    let white_material = materials.add(ColorMaterial::from(Color::WHITE));
    let gray_material = materials.add(ColorMaterial::from(Color::GRAY));

    // Spawn floor
    commands.spawn(MaterialMesh2dBundle {
        mesh: floor_mesh.clone(),
        material: gray_material.clone(),
        transform: Transform {
            translation: Vec3::new(0., -202., 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    // spawn wall
    commands.spawn(MaterialMesh2dBundle {
        mesh: floor_mesh.clone(),
        material: gray_material.clone(),
        transform: Transform {
            translation: Vec3::new(WALL_POSITION - 2., 0., 0.),
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn small cube
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: cube_mesh.clone(),
            material: white_material.clone(),
            transform: Transform {
                translation: Vec3::new(0., -190., 0.),
                scale: Vec3::ONE * CUBE_SCALE,
                ..Default::default()
            },
            ..Default::default()
        },
        Cube {
            mass: 1.,
            velocity: 0.,
        },
    ));

    // Spawn big cube
    let big_cube_mass: f32 = 100.;
    let big_cube_side = big_cube_mass.powf(1. / 3.);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: cube_mesh.clone(),
            material: white_material.clone(),
            transform: Transform {
                translation: Vec3::new(200., -200. + CUBE_SCALE * big_cube_side / 2., 0.),
                scale: Vec3::ONE * CUBE_SCALE * big_cube_side,
                ..Default::default()
            },
            ..Default::default()
        },
        Cube {
            mass: big_cube_mass,
            velocity: -50.,
        },
    ));
}

fn cube_velocity_system(time: Res<FixedTime>, mut query: Query<(&mut Transform, &Cube)>) {
    let dt = time.period.as_secs_f32();

    for (mut tf, cube) in query.iter_mut() {
        tf.translation.x += cube.velocity * dt;
    }
}

fn cube_collision_system(mut query: Query<(&Transform, &mut Cube)>) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([(tf1, mut c1), (tf2, mut c2)]) = combinations.fetch_next() {
        let collision = collide_aabb::collide(
            tf1.translation,
            tf1.scale.truncate(),
            tf2.translation,
            tf2.scale.truncate(),
        );

        if collision.is_some() {
            let v1 = (2. * c2.mass * c2.velocity + c1.velocity * (c1.mass - c2.mass))
                / (c1.mass + c2.mass);
            let v2 = (2. * c1.mass * c1.velocity + c2.velocity * (c2.mass - c1.mass))
                / (c1.mass + c2.mass);

            c1.velocity = v1;
            c2.velocity = v2;
        }
    }
}

fn wall_collision_system(mut query: Query<(&mut Transform, &mut Cube)>) {
    for (mut tf, mut cube) in query.iter_mut() {
        if tf.translation.x - tf.scale.x / 2. <= WALL_POSITION {
            tf.translation.x = WALL_POSITION + tf.scale.x / 2.;
            cube.velocity *= -1.;
        }
    }
}
