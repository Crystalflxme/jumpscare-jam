use bevy::prelude::*;
use avian3d::{math::Scalar, prelude::*};

const WATER: f32 = 0.0;
const BUOY: f32 = 1.8;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

pub fn character_system(
    time: Res<Time>,
    mut query: Query<(
        &MovementAcceleration,
        &mut LinearVelocity,
        &Transform
    ), With<Character>>
) {
    let delta_time = time.delta_seconds();

    for
        (
            movement_acceleration,
            mut linear_velocity,
            transform
        )
    in &mut query {
        let accel = movement_acceleration.0 * delta_time;
        let y = transform.translation.y;

        if y < 0.0 {
            linear_velocity.y += BUOY * accel * (WATER - y);
        }
    
        // linear_velocity.x += direction.x * accel;
        // linear_velocity.z += direction.z * accel;
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let boat_handle = meshes.add(Cylinder::new(1.5, 0.5));
    let boat_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.6, 0.1),
        ..default()
    });

    commands
        .spawn((
            PbrBundle {
                mesh: boat_handle.clone(),
                material: boat_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cylinder(1.5, 0.5),
            MovementAcceleration(10.0),
            Character,
        ));
}