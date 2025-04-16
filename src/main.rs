use std::ops::Mul;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::ExitCondition,
};

const CANNON_STEP: f32 = 100.;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Cannon {
    lives: u8,
    can_shoot: bool,
}

#[derive(Component)]
struct CannonBall;

enum AlienType {
    Squid,
    Crab,
    Octopus,
    UFO,
}

#[derive(Component)]
struct Alien {
    kind_of: AlienType,
    pos: Position,
    is_alive: bool,
    // score_value: u32, // should be randomized: 50, 100, 150, 300
}

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
        Sprite::from_image(asset_server.load("cannon.png")),
        Transform::from_xyz(0., bottom, 0.),
        Cannon {
            lives: 3,
            can_shoot: true,
        },
    ));

    for i in -3..4 {
        commands.spawn((
            Sprite::from_image(asset_server.load("squid.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(8.), 0.),
            Alien {
                kind_of: AlienType::Squid,
                pos: Position { x: sprite_pad.mul(i as f32), y: margin_top.mul(8.) },
                is_alive: true
            },
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("crab.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(4.), 0.),
            Alien {
                kind_of: AlienType::Crab,
                pos: Position { x: sprite_pad.mul(i as f32), y: margin_top.mul(4.) },
                is_alive: true
            },
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("octopus.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top, 0.),
            Alien {
                kind_of: AlienType::Octopus,
                pos: Position { x: sprite_pad.mul(i as f32), y: margin_top },
                is_alive: true
            },
        ));
    }

    commands.spawn((
        Sprite::from_image(asset_server.load("cannon_ball.png")),
        Transform::from_xyz(0., -margin_bottom.mul(2.), 0.),
        CannonBall,
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("ufo.png")),
        Transform::from_xyz(
            (-screen_width / 2.) + sprite_pad, // just for demonstraction
            (screen_height / 2.) - sprite_pad,
            0.,
        ),
    ));
}

fn apply_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cannon_transform: Single<&mut Transform, With<Cannon>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= CANNON_STEP;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += CANNON_STEP;
    }

    if keyboard_input.just_released(KeyCode::Space) {
        info!("kew ! kew 1");
        todo!("call/implement the shoot logic");
    }

    cannon_transform.translation.x += direction * time.delta_secs();
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
        .add_systems(Update, apply_user_input)
        .run();
}
