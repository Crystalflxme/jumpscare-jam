use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Rotating;

pub fn test_system(mut windows: Query<&mut Window>, time: Res<Time>) {
    let mut window = windows.single_mut();

    window.title = format!(
        "Seconds since startup: {}",
        time.elapsed().as_secs_f32()
    );
}

pub fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotating>>) {
    for mut transform in &mut query {
        transform.rotate_x(3.0 * time.delta_seconds());
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    let ocean_handle = meshes.add(Cuboid::new(1000.0, 2.0, 1000.0));
    let ocean_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgba(0.1, 0.1, 0.6, 0.8),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let sand_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.3),
        ..default()
    });

    // Rotating cubes
    commands
        .spawn((
            PbrBundle {
                mesh: cube_handle.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            Rotating,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: cube_handle.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            });
        });

    // Create ocean
    commands
        .spawn(
            PbrBundle {
                mesh: ocean_handle.clone(),
                material: ocean_material_handle.clone(),
                transform: Transform::from_xyz(0.0, -3.0, 0.0),
                ..default()
            }
        );

    // Create sand
    commands
        .spawn((
            PbrBundle {
                mesh: ocean_handle.clone(),
                material: sand_material_handle.clone(),
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
