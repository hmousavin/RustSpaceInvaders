use bevy::{
    prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::{ExitCondition, WindowResolution}
};

const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let image_paths = [
        "player_38.png",
        "green_3.png",
        "red_5.png",
        "yellow_5.png",
    ];

    for (_, path) in image_paths.iter().enumerate() {
        commands.spawn(Sprite::from_image(
            asset_server.load(*path),
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::PRIMARY), // or VULKAN
                        ..default()
                    }),
                    synchronous_pipeline_compilation: false,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            SCREEN_WIDTH,
                            SCREEN_HEIGHT,
                        )
                        .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    exit_condition: ExitCondition::OnPrimaryClosed,
                    close_when_requested: true,
                }),
        )
        // .add_plugins(GizmoPlugin)
        .add_systems(Startup, setup)
        .run();
}
