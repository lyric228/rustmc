use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::player::types::*;

/// Система для обработки движения мыши и поворота камеры/игрока
pub fn mouse_move_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut PlayerLook, &mut Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<WorldModelCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    // Чувствительность мыши
    const MOUSE_SENSITIVITY: f32 = 0.002;
    
    // Суммарное движение мыши за кадр
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }
    
    // Если нет движения мыши, выходим
    if delta.is_nan() || (delta.x == 0.0 && delta.y == 0.0) {
        return;
    }
    
    // Применяем поворот для всех сущностей игрока
    for (mut look, mut transform) in query.iter_mut() {
        // Вычисляем изменение углов поворота
        let delta_yaw = -delta.x * MOUSE_SENSITIVITY;
        let delta_pitch = -delta.y * MOUSE_SENSITIVITY;
        
        // Обновляем углы поворота
        look.yaw += delta_yaw;
        look.pitch += delta_pitch;
        
        // Ограничиваем углы наклона (pitch) чтобы нельзя было запрокинуть голову слишком сильно
        look.pitch = look.pitch.clamp(-1.5, 1.5); // примерно -85° до 85°
        
        // Применяем поворот по горизонтали (yaw) к трансформации игрока
        let yaw_rotation = Quat::from_rotation_y(look.yaw);
        transform.rotation = yaw_rotation;
        
        // Обновляем камеру, применяя и вертикальный наклон
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Сначала сбрасываем поворот камеры к базовому направлению
            camera_transform.rotation = Quat::IDENTITY;
            
            // Затем применяем поворот по вертикали (pitch)
            let pitch_rotation = Quat::from_rotation_x(look.pitch);
            
            // И по горизонтали (yaw) - должен совпадать с поворотом игрока
            camera_transform.rotation = yaw_rotation * pitch_rotation;
        }
    }
}
