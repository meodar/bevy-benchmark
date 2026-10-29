#[cfg(feature = "fps_debug")]
use bevy::prelude::*;

#[cfg(feature = "fps_debug")]
pub fn print_fps_system(diagnostics: Res<bevy::diagnostic::DiagnosticsStore>) {
    if let Some(fps) = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.average())
    {
        println!("FPS: {:.2}", fps);
    }
}
