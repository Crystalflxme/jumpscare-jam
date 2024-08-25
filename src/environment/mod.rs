use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Rotating;

pub fn window_system(mut windows: Query<&mut Window>, time: Res<Time>) {
    let mut window = windows.single_mut();

    window.title = format!(
        "Session time: {}",
        time.elapsed().as_secs_f32()
    );
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {    
    let plane_mesh = meshes.add(Cuboid::new(1000.0, 2.0, 1000.0));
    let ocean_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.1, 0.1, 0.6, 0.8),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let sand_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.3),
        ..default()
    });
    let rock_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.25),
        ..default()
    });
    let spinner_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.65, 0.65, 0.7),
        ..default()
    });

    // Platforms and spinners
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(8.0, 8.0, 8.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(0.5, -4.8, -9.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 8.0, 8.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(8.0, 8.0, 8.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(8.0, -4.2, -11.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 8.0, 8.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(8.0, 8.0, 8.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(8.0, -4.2, -22.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 8.0, 8.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(8.0, 8.0, 6.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(1.0, -3.0, -22.5),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 8.0, 6.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.5, 8.0, 1.5)),
            material: rock_material.clone(),
            transform: Transform::from_xyz(1.5, -1.5, -24.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.5, 8.0, 1.5),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(8.0, 8.0, 6.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(-5.0, 0.0, -22.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 8.0, 6.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(9.0, 3.0, 0.8)),
            material: spinner_material.clone(),
            transform: Transform::from_xyz(-9.0, 5.5, -22.0),
            ..default()
        },
        Rotating,
        RigidBody::Kinematic,
        Collider::cuboid(9.0, 3.0, 0.8),
        AngularVelocity(Vec3::Y * -2.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(8.0, 8.0, 6.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(-13.0, 0.0, -22.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 8.0, 6.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5.0, 8.0, 5.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(-17.0, -1.0, -14.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(5.0, 8.0, 5.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.5, 1.5, 5.0)),
            material: rock_material.clone(),
            transform: Transform::from_xyz(-17.0, 3.5, -10.5),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.5, 1.5, 5.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5.0, 8.0, 5.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(-18.5, -0.5, -2.5),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(5.0, 8.0, 5.0),
    ));

    // Create ocean
    commands.spawn(
        PbrBundle {
            mesh: plane_mesh.clone(),
            material: ocean_material.clone(),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        }
    );

    // Create sand
    commands.spawn((
        PbrBundle {
            mesh: plane_mesh.clone(),
            material: sand_material.clone(),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1000.0, 2.0, 1000.0),
    ));

    // Create light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
