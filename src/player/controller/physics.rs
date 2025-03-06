use crate::player::types::*;
use bevy::prelude::*;

/// Система обработки ввода с учетом поворота игрока
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut AccumulatedInput,
            &mut Velocity,
            &Transform,
            &PlayerLook,
        ),
        With<Player>,
    >,
) {
    const SPEED: f32 = 16.0;
    for (mut input, mut velocity, transform, look) in query.iter_mut() {
        // Сбрасываем ввод
        input.x = 0.0;
        input.y = 0.0;

        // Накапливаем ввод с клавиатуры
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.y += 1.0; // Вперед
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.y -= 1.0; // Назад
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.x -= 1.0; // Влево
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.x += 1.0; // Вправо
        }

        // Нормализация вектора ввода для диагонального движения с той же скоростью
        let movement_vec = Vec2::new(input.x, input.y);
        let movement = if movement_vec.length_squared() > 0.0 {
            movement_vec.normalize()
        } else {
            movement_vec
        };

        // Получаем векторы направления из трансформации игрока
        let forward = Vec3::new(0.0, 0.0, -1.0); // Базовый вектор "вперед"
        let right = Vec3::new(1.0, 0.0, 0.0); // Базовый вектор "вправо"

        // Преобразуем базовые векторы с учетом поворота персонажа
        // Используем только поворот по горизонтали (yaw), игнорируя вертикальный наклон (pitch)
        let forward_world = transform.rotation * forward;
        let right_world = transform.rotation * right;

        // Проецируем векторы на горизонтальную плоскость (убираем Y-компоненту)
        // и нормализуем, чтобы сохранить постоянную скорость
        let forward_world = Vec3::new(forward_world.x, 0.0, forward_world.z).normalize_or_zero();
        let right_world = Vec3::new(right_world.x, 0.0, right_world.z).normalize_or_zero();

        // Вычисляем итоговое направление движения
        let direction = (forward_world * movement.y + right_world * movement.x).normalize_or_zero();

        // Устанавливаем скорость
        velocity.0 = direction * SPEED;
    }
}

/// Система физического перемещения объектов
pub fn advance_physics(
    time: Res<Time>,
    mut query: Query<(
        &Velocity,
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
    )>,
) {
    let delta_seconds = time.delta_seconds();
    for (velocity, mut position, mut prev_position) in query.iter_mut() {
        // Запоминаем текущую позицию
        prev_position.0 = position.0;

        // Обновляем позицию на основе скорости
        position.0 += velocity.0 * delta_seconds;
    }
}

/// Система применения трения для замедления объектов
pub fn apply_friction(mut query: Query<&mut Velocity>, time: Res<Time>) {
    let friction = 0.95_f32;
    for mut velocity in query.iter_mut() {
        velocity.0 *= friction.powf(time.delta_seconds() * 60.0);

        // Останавливаем объект, если скорость очень низкая
        if velocity.0.length_squared() < 0.001 {
            velocity.0 = Vec3::ZERO;
        }
    }
}

/// Система интерполяции визуальной позиции для плавного отображения
pub fn interpolate_rendered_transform(
    time: Res<Time>,
    mut query: Query<(
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
        &mut Transform,
    )>,
) {
    let alpha = time.delta_seconds() / Time::FIXED_TIME_STEP;
    let alpha = alpha.clamp(0.0, 1.0);

    for (position, prev_position, mut transform) in query.iter_mut() {
        // Интерполируем позицию между предыдущей и текущей физической позицией
        transform.translation = prev_position.0.lerp(position.0, alpha);
    }
}
