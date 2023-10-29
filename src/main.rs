use bevy::{a11y::AccessibilityPlugin, input::InputPlugin, prelude::*, winit::WinitPlugin};

fn main() {
    App::new()
        .add_plugins((
            WindowPlugin::default(),
            WinitPlugin,
            InputPlugin,
            AccessibilityPlugin,
        ))
        .run();
}
