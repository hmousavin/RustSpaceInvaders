use std::ops::Mul;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::ExitCondition,
};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let window = windows.single(); // assumes one window
    let screen_width = window.resolution.width();
    let screen_height = window.resolution.height();
    let margin_top: f32 = screen_height / 30.;
    let margin_bottom: f32 = screen_height / 30.;
    let sprite_pad: f32 = screen_width / 10.;

    commands.spawn(Camera2d);

    let bottom = (-screen_height / 2.) + margin_bottom;
    commands.spawn((
        Sprite::from_image(asset_server.load("player_38.png")),
        Transform::from_xyz(0., bottom, 0.),
    ));

    for i in -3..4 {
        commands.spawn((
            Sprite::from_image(asset_server.load("green_3.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(8.), 0.),
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("yellow_5.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(4.), 0.),
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("red_5.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top, 0.),
        ));
    }

    commands.spawn((
        Sprite::from_image(asset_server.load("white_arrow.png")),
        Transform::from_xyz(0., -margin_bottom.mul(2.), 0.),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("extra.png")),
        Transform::from_xyz(
            (-screen_width / 2.) + sprite_pad, // just for demonstraction
            (screen_height / 2.) - sprite_pad,
            0.,
        ),
    ));
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
                    exit_condition: ExitCondition::OnPrimaryClosed,
                    close_when_requested: true,
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup)
        .run();
}
