use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};

fn main() {
    App::new().add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: WgpuSettings {
            backends: None,
            ..default()
        }
        .into(),
        synchronous_pipeline_compilation: false, // or true if you want synchronous
    })).run();
}