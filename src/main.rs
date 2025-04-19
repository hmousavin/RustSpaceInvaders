use std::ops::Mul;

use bevy::{
    prelude::*, render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    }, window::ExitCondition
};

const CANNON_STEP: f32 = 10.;

#[derive(Component)]
struct BoundingBox {
    width: f32,
    height: f32
}

#[derive(Component)]
struct Cannon {
}

#[derive(Copy, Clone)]
enum BallType {
    CannonBall,
}

#[derive(Component)]
struct Ball {
    kind_of: BallType,
}

#[derive(Copy, Clone)]
enum AlienType {
    Squid,
    Crab,
    Octopus,
    UFO,
}

#[derive(Component)]
struct Alien {
    kind_of: AlienType,
    // score_value: u32, // should be randomized: 50, 100, 150, 300
}

fn get_alien_bounding_box(alien_type: AlienType) -> BoundingBox {
    let bb =  match alien_type {
        AlienType::Squid => BoundingBox { width: 40., height: 32. },
        AlienType::Crab => BoundingBox { width: 40., height: 32. },
        AlienType::Octopus => BoundingBox { width: 40., height: 32. },
        AlienType::UFO => BoundingBox { width: 40., height: 20. },
    };

    bb
}

fn get_ball_bounding_box(ball_type: BallType) -> BoundingBox {
    let bb = match ball_type {
        BallType::CannonBall => BoundingBox { width: 4., height: 10. },
    };

    bb
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left, Right
}

#[derive(Resource)]
struct AlienMoveDirection {
    dir: Direction
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
        Cannon{}
    ));

    for i in -3..4 {
        commands.spawn((
            Sprite::from_image(asset_server.load("squid.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(8.), 0.),
            Alien {
                kind_of: AlienType::Squid,
            },
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("crab.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top.mul(4.), 0.),
            Alien {
                kind_of: AlienType::Crab,
            },
        ));

        commands.spawn((
            Sprite::from_image(asset_server.load("octopus.png")),
            Transform::from_xyz(sprite_pad.mul(i as f32), margin_top, 0.),
            Alien {
                kind_of: AlienType::Octopus,
            },
        ));
    }

    commands.spawn((
        Sprite::from_image(asset_server.load("ufo.png")),
        Transform::from_xyz(
            (-screen_width / 2.) + sprite_pad, // just for demonstraction
            (screen_height / 2.) - sprite_pad,
            0.,
        ),
        Alien {
            kind_of: AlienType::UFO,
        }
    ));
}

fn handle_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cannon_transform: Single<&mut Transform, With<Cannon>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        cannon_transform.translation.x -= CANNON_STEP;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        cannon_transform.translation.x += CANNON_STEP;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            Sprite::from_image(asset_server.load("cannon_ball.png")),
            Transform::from_xyz(cannon_transform.translation.x, cannon_transform.translation.y+20., 0.),
            Ball {
                kind_of: BallType::CannonBall,
            },
        ));
    }
}

fn refresh_aliens(
    mut alien_transforms: Query<&mut Transform, With<Alien>>,
    mut alien_direction: ResMut<AlienMoveDirection>, 
    window: Single<&Window>,
) {
    let screen_left = -window.width() / 2.;
    let screen_right = window.width() / 2.;
    
    for mut transform in alien_transforms.iter_mut() {
        if alien_direction.dir == Direction::Right && transform.translation.x + 20. >= screen_right {
            alien_direction.dir = Direction::Left;
            break;
        }
        if alien_direction.dir == Direction::Left && transform.translation.x - 20. <= screen_left {
            alien_direction.dir = Direction::Right;
            break;
        }
    
        if alien_direction.dir == Direction::Left {
            transform.translation.x -= 2.;
        }
        else {
            transform.translation.x += 2.;
        }
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
            // BallType::AlienBall => -5.0,
        };

        // Despawn if off screen
        if transform.translation.y > window.height() / 2.0
            || transform.translation.y < -window.height() / 2.0
        {
            commands.entity(ball_entity).despawn();
        }
    }
}

// an implementation of Axis-Aligned Bounding Box
fn check_collisions(
    mut commands: Commands,
    balls: Query<(Entity, &Transform, &Ball)>,
    aliens: Query<(Entity, &Transform, &Alien)>,
) {
    for (ball_entity, ball_transform, ball) in balls.iter() {
        for (alien_entity, alien_transform, alien) in aliens.iter() {

            let ball_pos = ball_transform.translation;
            let alien_pos = alien_transform.translation;

            let ball_min_x = ball_pos.x - get_ball_bounding_box(ball.kind_of.clone()).width / 2.0;
            let ball_max_x = ball_pos.x + get_ball_bounding_box(ball.kind_of.clone()).width / 2.0;
            let ball_min_y = ball_pos.y - get_ball_bounding_box(ball.kind_of.clone()).height / 2.0;
            let ball_max_y = ball_pos.y + get_ball_bounding_box(ball.kind_of.clone()).height / 2.0;

            let alien_min_x = alien_pos.x - get_alien_bounding_box(alien.kind_of).width / 2.0;
            let alien_max_x = alien_pos.x + get_alien_bounding_box(alien.kind_of).width / 2.0;
            let alien_min_y = alien_pos.y - get_alien_bounding_box(alien.kind_of).height / 2.0;
            let alien_max_y = alien_pos.y + get_alien_bounding_box(alien.kind_of).height / 2.0;

            if ball_min_x < alien_max_x &&
            ball_max_x > alien_min_x &&
            ball_min_y < alien_max_y &&
            ball_max_y > alien_min_y
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
        .insert_resource(AlienMoveDirection{dir: Direction::Right})
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_inputs, refresh_aliens, refresh_balls, check_collisions))
        .run();
}
