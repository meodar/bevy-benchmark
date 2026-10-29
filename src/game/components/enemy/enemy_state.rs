use bevy::prelude::*;

/// Время жизни врага: от него считаются фазы траекторий Cos/Sin
#[derive(Component, Clone, Default)]
pub struct TimeElapsed(pub f64);

/// Позиция чтения в таблице чисел для нагрузки врага
#[derive(Component, Clone, Default)]
pub struct WorkloadCursor(pub usize);
