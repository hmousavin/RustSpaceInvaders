use std::ops::Mul;

use bevy::{
    prelude::*, render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    }, window::ExitCondition
};

const CANNON_STEP: f32 = 10.;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Cannon {
    pos: Position,
    lives: u8,
}

enum BallType {
    CannonBall,
    AlienBall,
}

#[derive(Component)]
struct Ball {
    kind_of: BallType,
    pos: Position, 
}

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
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
            pos: Position { x: 0., y: bottom },
            lives: 3,
        },
    ));

    for i in -3..4 {
        commands.spawn((
            Sprite::from_image(asset_server.load("squid.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(8.), 0.),
            Alien {
                kind_of: AlienType::Squid,
                pos: Position {
                    x: sprite_pad.mul(i as f32),
                    y: margin_top.mul(8.),
                },
                is_alive: true,
            },
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("crab.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(4.), 0.),
            Alien {
                kind_of: AlienType::Crab,
                pos: Position {
                    x: sprite_pad.mul(i as f32),
                    y: margin_top.mul(4.),
                },
                is_alive: true,
            },
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("octopus.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top, 0.),
            Alien {
                kind_of: AlienType::Octopus,
                pos: Position {
                    x: sprite_pad.mul(i as f32),
                    y: margin_top,
                },
                is_alive: true,
            },
        ));
    }

    commands.spawn((
        Sprite::from_image(asset_server.load("cannon_ball.png")),
        Transform::from_xyz(0., -margin_bottom.mul(2.), 0.),
        Ball {
            kind_of: BallType::CannonBall,
            pos: Position { x: 0., y: -margin_bottom.mul(2.) }
        },
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("ufo.png")),
        Transform::from_xyz(
            (-screen_width / 2.) + sprite_pad, // just for demonstraction
            (screen_height / 2.) - sprite_pad,
            0.,
        ),
        Alien {
            kind_of: AlienType::UFO,
            pos: Position {
                x: (-screen_width / 2.) + sprite_pad, // just for demonstraction
                y: (screen_height / 2.) - sprite_pad,
            },
            is_alive: true,
        }
    ));
}

fn handle_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cannon_transform: Single<&mut Transform, With<Cannon>>,
    mut cannon: Single<&mut Cannon>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        cannon_transform.translation.x -= CANNON_STEP;
        cannon.pos.x -= CANNON_STEP;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        cannon_transform.translation.x += CANNON_STEP;
        cannon.pos.x += CANNON_STEP;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            Sprite::from_image(asset_server.load("cannon_ball.png")),
            Transform::from_xyz(cannon.pos.x, cannon.pos.y+20., 0.),
            Ball {
                kind_of: BallType::CannonBall,
                pos: Position{x:cannon.pos.x, y:cannon.pos.y+20.},
            },
        ));
    }
}

fn refresh_aliens(
    mut alien_transforms: Query<&mut Transform, With<Alien>>,
    // window: Single<&Window>,
) {
    // let half_width = window.width();

    for mut transform in alien_transforms.iter_mut() {
        transform.translation.x += 2.;
    }
}

fn refresh_balls(
    mut commands: Commands,
    mut balls: Query<(Entity, &Ball, &mut Transform)>,
    window: Single<&Window>,
) {
    for (ball_entity, ball, mut transform) in balls.iter_mut() {
        // Move the ball
        transform.translation.y += match ball.kind_of {
            BallType::CannonBall => 5.0,
            BallType::AlienBall => -5.0,
        };

        // Despawn if off screen
        if transform.translation.y > window.height() / 2.0
            || transform.translation.y < -window.height() / 2.0
        {
            commands.entity(ball_entity).despawn();
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    balls: Query<(Entity, &Transform), With<Ball>>,
    aliens: Query<(Entity, &Transform), With<Alien>>,
) {
    for (ball_entity, ball_transform) in balls.iter() {
        for (alien_entity, alien_transform) in aliens.iter() {
            if ball_transform
                .translation
                .distance(alien_transform.translation)
                < 5.0
            {
                commands.entity(alien_entity).despawn();
                commands.entity(ball_entity).despawn();
                break; // prevent double-despawning same ball
            }
        }
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
                    exit_condition: ExitCondition::OnPrimaryClosed,
                    close_when_requested: true,
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_inputs, refresh_aliens, refresh_balls, check_collisions))
        .run();
}
