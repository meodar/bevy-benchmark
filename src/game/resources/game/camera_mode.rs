#[derive(Clone)]
pub enum CameraMode {
    Top { last_side_pos: f32 },
    Side { last_top_pos: f32 },
}

impl Default for CameraMode {
    fn default() -> Self {
        Self::Top { last_side_pos: 0.0 }
    }
}
