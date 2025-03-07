use bevy::prelude::*;
use crate::player::types::*;

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
            &mut AccumulatedInput,
            &mut Velocity,
            &Transform,
        &PlayerLook
    ), With<Player>>,
)
{
    const SPEED: f32 = 16.0;
    for (mut input, mut velocity, transform, _look) in query.iter_mut() {
        // Reset input
        input.x = 0.0;
        input.y = 0.0;

        // Accumulate keyboard input
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.y += 1.0; // Forward
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.y -= 1.0; // Backward
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.x -= 1.0; // Left
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.x += 1.0; // Right
        }

        // Normalize input vector for consistent diagonal movement speed
        let movement_vec = Vec2::new(input.x, input.y);
        let movement = if movement_vec.length_squared() > 0.0 {
            movement_vec.normalize()
        } else {
            movement_vec
        };

        // Get direction vectors from player transform
        // Using only horizontal rotation (yaw) to determine movement direction
        let forward = Vec3::new(0.0, 0.0, -1.0); // Base "forward" vector
        let right = Vec3::new(1.0, 0.0, 0.0);    // Base "right" vector

        // Transform base vectors according to player rotation
        let forward_world = transform.rotation * forward;
        let right_world = transform.rotation * right;

        // Project vectors to horizontal plane and normalize
        let forward_world = Vec3::new(forward_world.x, 0.0, forward_world.z).normalize_or_zero();
        let right_world = Vec3::new(right_world.x, 0.0, right_world.z).normalize_or_zero();

        // Calculate final movement direction
        let direction = (forward_world * movement.y + right_world * movement.x).normalize_or_zero();

        // Apply velocity
        velocity.0 = direction * SPEED;
    }
}

pub fn advance_physics(
    time: Res<Time>,
    mut query: Query<(
        &Velocity,
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
    )>,
)
{
    let delta_seconds = time.delta_secs();
    for (velocity, mut position, mut prev_position) in query.iter_mut() {
        // Remember current position
        prev_position.0 = position.0;

        // Update position based on velocity
        position.0 += velocity.0 * delta_seconds;
    }
}

pub fn apply_friction(
    mut query: Query<&mut Velocity>,
)
{
    const FRICTION: f32 = 0.9;
    for mut velocity in query.iter_mut() {
        velocity.0 *= FRICTION;
    }
}
pub fn interpolate_rendered_transform(
    mut query: Query<(
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
        &mut Transform,
    )>,
)
{
    // Interpolation factor (could be time-based in more complex implementation)
    const ALPHA: f32 = 0.5;
    for (current, previous, mut transform) in query.iter_mut() {
        // Linear interpolation between previous and current position
        transform.translation = previous.0.lerp(current.0, ALPHA);
    }
}
