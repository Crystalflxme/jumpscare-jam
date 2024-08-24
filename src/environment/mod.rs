use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Rotating;

#[derive(Component)]
pub struct RotationSpeed(f32);

pub fn test_system(mut windows: Query<&mut Window>, time: Res<Time>) {
    let mut window = windows.single_mut();

    window.title = format!(
        "Seconds since startup: {}",
        time.elapsed().as_secs_f32()
    );
}

pub fn spinner_system(mut query: Query
    <(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &RotationSpeed
    ), With<Rotating>>
) {
    for (
        mut linear_velocity,
        mut angular_velocity,
        rotation_speed
    ) in &mut query {
        linear_velocity.0 = Vec3::ZERO;
        angular_velocity.0 = -Vec3::Z * rotation_speed.0;
    }
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
    let spinner_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.65, 0.65, 0.7),
        ..default()
    });

    // Spinners
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(6.0, 1.0, 2.0)),
            material: spinner_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Rotating,
        RigidBody::Dynamic,
        Collider::cuboid(6.0, 0.8, 2.0),
        AngularVelocity(Vec3::ZERO),
        LinearVelocity(Vec3::ZERO),
        RotationSpeed(4.0),

        // TODO: Fix rotation consistency
    ));
    
    // Islands
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(8.0, 8.0, 8.0)),
            material: sand_material.clone(),
            transform: Transform::from_xyz(0.0, -4.8, -9.0),
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
            transform: Transform::from_xyz(8.0, -4.2, -23.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 8.0, 8.0),
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
    
    // Create ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0, 1.0, 1.0),
        brightness: 0.02,
    });

    // Create light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
