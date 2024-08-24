use bevy::prelude::*;
use avian3d::{math::Scalar, prelude::*};

const CAMERA_OFFSET: Transform = Transform::from_xyz(0.0, 10.0, 8.0);
const XZ_AXIS: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 1.0};
const SPEED: f32 = 5.0;
const WATER: f32 = 0.0;
const BUOY: f32 = 1.8;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

pub fn character_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &MovementAcceleration,
            &mut LinearVelocity,
            &Transform
        ),
        With<Character>,
    >,
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

        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW)
        {
            direction -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyS)
        {
            direction += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyA)
        {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::KeyD)
        {
            direction += Vec3::X;
        }

        let mut drain = Vec3::ZERO;
        let dir = direction.normalize_or_zero();
        if dir.length() <= f32::EPSILON && linear_velocity.length() > f32::EPSILON {
            drain = XZ_AXIS * -linear_velocity.signum() * SPEED * delta_time;
        }
        
        linear_velocity.x = f32::clamp(
            linear_velocity.x + dir.x * accel * SPEED + drain.x,
            -SPEED,
            SPEED,
        );
        linear_velocity.z = f32::clamp(
            linear_velocity.z + dir.z * accel * SPEED + drain.z,
            -SPEED,
            SPEED,
        );
    }
}

pub fn camera_system(
    mut set: ParamSet<(
        Query<&mut Transform, With<Character>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let mut character_position: Option<Vec3> = None;
    for character_transform in &set.p0() {
        character_position = Some(character_transform.translation);
    }

    if character_position.is_none() {
        return;
    }

    for mut camera_transform in &mut set.p1() {
        camera_transform.translation = character_position.unwrap()
            * XZ_AXIS
            + CAMERA_OFFSET.translation;
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

    commands.spawn((
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

    commands.spawn((
        Camera3dBundle {
            transform: CAMERA_OFFSET.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}