use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use super::Player;

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player {
            movement_speed: 5.0,
            mouse_sensitivity: 0.002,
        },
    ));
}

pub fn movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, player) = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    // Forward/Backward
    if keyboard.pressed(KeyCode::W) {
        direction += transform.forward();
    }
    if keyboard.pressed(KeyCode::S) {
        direction += transform.back();
    }

    // Left/Right
    if keyboard.pressed(KeyCode::A) {
        direction += transform.left();
    }
    if keyboard.pressed(KeyCode::D) {
        direction += transform.right();
    }

    // Up/Down
    if keyboard.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        direction += Vec3::NEG_Y;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * player.movement_speed * time.delta_seconds();
    }
}

pub fn mouse_input(
    mut windows: Query<&mut Window>,
    mut player_query: Query<(&mut Transform, &Player)>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button: Res<Input<MouseButton>>,
) {
    let mut window = windows.single_mut();
    
    // Захват курсора при нажатии ЛКМ
    if mouse_button.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    let (mut transform, player) = player_query.single_mut();

    for mouse_event in mouse_events.iter() {
        if window.cursor.grab_mode == CursorGrabMode::Locked {
            // Поворот камеры
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            
            yaw -= mouse_event.delta.x * player.mouse_sensitivity;
            pitch -= mouse_event.delta.y * player.mouse_sensitivity;
            
            pitch = pitch.clamp(-1.54, 1.54); // Примерно ±88 градусов
            
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        }
    }

    // Освобождение курсора по Escape
    if keyboard.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub fn block_interaction(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    // Здесь можно добавить raycast для определения блока под прицелом
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        // Логика разрушения блока
    }
    
    if mouse_button.just_pressed(MouseButton::Right) {
        // Логика размещения блока
    }
}
