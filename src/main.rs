use std::{fs, time::Duration};

use bevy::{
    prelude::*, render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    }, window::ExitCondition
};
use rand::seq::IndexedRandom;
use ron::from_str;
use serde::Deserialize;

const CANNON_STEP: f32 = 10.;
const BASE_ALIEN_SCORE: usize = 10;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Playing,
    GameOver,
    Victory,
}

#[derive(Event)]
struct AppStateEvent(AppState);

enum MissionResult {
    Restart, 
    Advance,
}

#[derive(Event)]
struct ChangeMissionEvent(MissionResult);

#[derive(Component)]
struct BoundingBox {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Cannon {
    lives: usize,
    score: usize,
}

#[derive(Copy, Clone, PartialEq)]
enum BallType {
    CannonBall,
    AlienBall,
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
    score_value: usize, 
}

fn get_alien_bounding_box(alien_type: AlienType) -> BoundingBox {
    let bb = match alien_type {
        AlienType::Squid => BoundingBox {
            width: 40.,
            height: 32.,
        },
        AlienType::Crab => BoundingBox {
            width: 40.,
            height: 32.,
        },
        AlienType::Octopus => BoundingBox {
            width: 40.,
            height: 32.,
        },
        AlienType::UFO => BoundingBox {
            width: 40.,
            height: 20.,
        },
    };

    bb
}

fn get_ball_bounding_box(ball_type: BallType) -> BoundingBox {
    let bb = match ball_type {
        BallType::CannonBall | BallType::AlienBall => BoundingBox {
            width: 4.,
            height: 10.,
        },
    };

    bb
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Resource)]
struct AlienMoveDirection {
    dir: Direction,
}

#[derive(Resource)]
struct AlienWaitToShoot {
    secs: Timer,
}

#[derive(Resource)]
struct Difficulty {
    level: usize
}

#[derive(PartialEq, Eq)]
enum EventTypes {
    CannonLives,
    Score
}

#[derive(Event)]
struct HudEvent(EventTypes, usize);

#[derive(Component)]
struct HudLivesText;

#[derive(Component)]
struct HudScoreText;

#[derive(Component)]
struct HudLevelText;

fn get_sprite_from_symbol(sym: char, x: f32, y: f32, asset_server: &AssetServer) -> (Sprite, Transform, Alien)  {
    let sprite = match sym {
        'S' => Sprite::from_image(asset_server.load("squid.png")),
        'C' => Sprite::from_image(asset_server.load("crab.png")),
        'O' => Sprite::from_image(asset_server.load("octopus.png")),
        'U' => Sprite::from_image(asset_server.load("ufo.png")),
        _ => panic!("Unrecognized symbol: {}", sym),
    };

    let transform = Transform::from_xyz(x, y, 0.);

    let alien = match sym {
        'S' => Alien {
            kind_of: AlienType::Squid,
            score_value: 1 * BASE_ALIEN_SCORE,
        },
        'C' => Alien {
            kind_of: AlienType::Crab,
            score_value: 2 * BASE_ALIEN_SCORE,
        },
        'O' => Alien {
            kind_of: AlienType::Octopus,
            score_value: 3 * BASE_ALIEN_SCORE,
        },
        'U' => Alien {
            kind_of: AlienType::UFO,
            score_value: 5 * BASE_ALIEN_SCORE,
        },
        _ => panic!("Unrecognized symbol: {}", sym),

    };

    (sprite, transform, alien)
}

#[derive(Debug, Deserialize)]
struct AlienWave {
    pattern: Vec<Vec<char>>,
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    difficulty: Res<Difficulty>,
    window: Single<&Window>,) 
{
    commands.spawn(Camera2d);

    render_sprites(&mut commands, &asset_server, difficulty.level, &window);
    
    write_on_hud(commands, "lives: 3", "score: 0", "level: 1");
}

fn write_on_hud(mut commands: Commands<'_, '_>, lives_text: &str, score_text: &str, level_text: &str) {
    let hud_root = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .id();

    let lives = commands.spawn((Text::new(lives_text), HudLivesText)).id();
    let score = commands.spawn((Text::new(score_text), HudScoreText)).id();
    let level = commands.spawn((Text::new(level_text), HudLevelText)).id();
    commands.entity(hud_root).add_children(&[lives, score, level]);
}

fn render_sprites(commands: &mut Commands<'_, '_>, asset_server: &AssetServer, difficulty_level: usize, window: &Window) {
    let screen_width = window.resolution.width();
    let screen_height = window.resolution.height();
    let margin_top: f32 = screen_height / 30.;
    let margin_bottom: f32 = screen_height / 30.;
    let sprite_pad: f32 = screen_width / 10.;
    let bottom = (-screen_height / 2.) + margin_bottom;
    
    commands.spawn((
        Sprite::from_image(asset_server.load("cannon.png")),
        Transform::from_xyz(0., bottom, 0.),
        Cannon { lives: 3, score: 0 },
    ));

    let ron_string = fs::read_to_string("assets/levels.ron").unwrap();
    let assets: Vec<AlienWave> = from_str(&ron_string).unwrap();
    
    for (row_idx, row) in assets[difficulty_level - 1].pattern.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '.' { continue; } // skip empty space

            let x = (col_idx as f32 - row.len() as f32 / 2.0) * sprite_pad;
            let y = screen_height / 2.0 - margin_top - (row_idx as f32 * sprite_pad);

            let (sprite, transform, alien) = get_sprite_from_symbol(ch, x, y, &asset_server);
            commands.spawn((sprite, transform, alien));
        }
    }
}

fn handle_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cannon_transform: Single<&mut Transform, With<Cannon>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        if -window.width() / 2. < cannon_transform.translation.x - CANNON_STEP {
            cannon_transform.translation.x -= CANNON_STEP;
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        if cannon_transform.translation.x + CANNON_STEP < window.width() / 2. {
            cannon_transform.translation.x += CANNON_STEP;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            Sprite::from_image(asset_server.load("cannon_ball.png")),
            Transform::from_xyz(
                cannon_transform.translation.x,
                cannon_transform.translation.y + 20.,
                0.,
            ),
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
        if alien_direction.dir == Direction::Right && transform.translation.x + 20. >= screen_right
        {
            alien_direction.dir = Direction::Left;
            break;
        }
        if alien_direction.dir == Direction::Left && transform.translation.x - 20. <= screen_left {
            alien_direction.dir = Direction::Right;
            break;
        }

        match alien_direction.dir {
            Direction::Left => transform.translation.x -= 2.,
            Direction::Right => transform.translation.x += 2.,
        }
    }
}

fn aliens_attack(
    mut commands: Commands,
    mut shoot_timer: ResMut<AlienWaitToShoot>,
    time: Res<Time>,
    aliens_query: Query<(&Transform, Entity), With<Alien>>,
    asset_server: Res<AssetServer>,
) {
    shoot_timer.secs.tick(time.delta());

    if shoot_timer.secs.just_finished() {
        let mut rng = rand::rng();
        let aliens: Vec<(&Transform, Entity)> = aliens_query.iter().collect();

        if let Some((transform, _)) = aliens.choose(&mut rng) {
            commands.spawn((
                Sprite::from_image(asset_server.load("alien_ball.png")),
                Transform::from_xyz(transform.translation.x, transform.translation.y - 25.0, 0.0),
                Ball {
                    kind_of: BallType::AlienBall,
                },
            ));
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

// an implementation of Axis-Aligned Bounding Box
fn check_collisions(
    mut commands: Commands,
    balls: Query<(Entity, &Transform, &Ball)>,
    aliens: Query<(Entity, &Transform, &Alien)>,
    mut single_cannon: Single<(Entity, &Transform, &mut Cannon)>,
    mut hud_ew: EventWriter<HudEvent>,
    mut mission_ew: EventWriter<ChangeMissionEvent>,
) {
    for (ball_entity, ball_transform, ball) in balls.iter() {
        let ball_pos = ball_transform.translation;

        let ball_min_x = ball_pos.x - get_ball_bounding_box(ball.kind_of.clone()).width / 2.0;
        let ball_max_x = ball_pos.x + get_ball_bounding_box(ball.kind_of.clone()).width / 2.0;
        let ball_min_y = ball_pos.y - get_ball_bounding_box(ball.kind_of.clone()).height / 2.0;
        let ball_max_y = ball_pos.y + get_ball_bounding_box(ball.kind_of.clone()).height / 2.0;

        match ball.kind_of {
            BallType::CannonBall => {
                for (alien_entity, alien_transform, alien) in aliens.iter() {
                    let alien_pos = alien_transform.translation;

                    let alien_min_x =
                        alien_pos.x - get_alien_bounding_box(alien.kind_of).width / 2.0;
                    let alien_max_x =
                        alien_pos.x + get_alien_bounding_box(alien.kind_of).width / 2.0;
                    let alien_min_y =
                        alien_pos.y - get_alien_bounding_box(alien.kind_of).height / 2.0;
                    let alien_max_y =
                        alien_pos.y + get_alien_bounding_box(alien.kind_of).height / 2.0;

                    if aabb_collision(
                        ball_min_x,
                        ball_max_x,
                        ball_min_y,
                        ball_max_y,
                        alien_min_x,
                        alien_max_x,
                        alien_min_y,
                        alien_max_y,
                    ) {
                        commands.entity(alien_entity).despawn();
                        commands.entity(ball_entity).despawn();

                        single_cannon.2.score += alien.score_value;
                        hud_ew.send(HudEvent(EventTypes::Score, single_cannon.2.score));

                        break;
                    }
                }
            
                // if aliens.is_empty() {
                //     mission_ew.send(ChangeMissionEvent(MissionResult::Advance));
                // }
            }

            BallType::AlienBall => {
                let (_, cannon_transform, cannon) = &mut *single_cannon;
                let cannon_pos = cannon_transform.translation;

                let cannon_bb = BoundingBox {
                    width: 60.0,
                    height: 30.0,
                }; // Replace with actual values if you store them

                let cannon_min_x = cannon_pos.x - cannon_bb.width / 2.0;
                let cannon_max_x = cannon_pos.x + cannon_bb.width / 2.0;
                let cannon_min_y = cannon_pos.y - cannon_bb.height / 2.0;
                let cannon_max_y = cannon_pos.y + cannon_bb.height / 2.0;

                if aabb_collision(
                    ball_min_x,
                    ball_max_x,
                    ball_min_y,
                    ball_max_y,
                    cannon_min_x,
                    cannon_max_x,
                    cannon_min_y,
                    cannon_max_y,
                ) {
                    commands.entity(ball_entity).despawn();

                    if cannon.lives > 0 {
                        cannon.lives = cannon.lives - 1;
                        hud_ew.send(HudEvent(EventTypes::CannonLives, cannon.lives));
                    }
                    else {
                        mission_ew.send(ChangeMissionEvent(MissionResult::Restart));
                    }
                    break;
                }
            }
        }
    }
}

fn aabb_collision(
    ball_min_x: f32,
    ball_max_x: f32,
    ball_min_y: f32,
    ball_max_y: f32,
    alien_min_x: f32,
    alien_max_x: f32,
    alien_min_y: f32,
    alien_max_y: f32,
) -> bool {
    if ball_min_x < alien_max_x
        && ball_max_x > alien_min_x
        && ball_min_y < alien_max_y
        && ball_max_y > alien_min_y
    {
        return true;
    }

    false
}

fn refresh_hud(
    mut ev_list: EventReader<HudEvent>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<HudLivesText>>,
        Query<&mut Text, With<HudScoreText>>,
    )>,
) {
    for HudEvent(e_type, e_val) in ev_list.read() {
        match e_type {
            EventTypes::CannonLives => {
                if let Ok(mut text) = text_params.p0().get_single_mut() {
                    text.0 = format!("lives: {}", e_val);
                }
            }
            EventTypes::Score => {
                if let Ok(mut text) = text_params.p1().get_single_mut() {
                    text.0 = format!("score: {}", e_val);
                }
            }
        }
    }
}

fn toggle_system_state(
    mut ev_list: EventReader<AppStateEvent>, 
    mut commands: Commands, 
    mut next_state: ResMut<NextState<AppState>>,
    mut difficulty: ResMut<Difficulty>,) 
{
    for e_type in ev_list.read() {
        let text = Text::new(
            match e_type.0 {
                AppState::GameOver => "Game Over",
                AppState::Victory => "You win !", 
                _ => "",
            }
        );
        
        commands.spawn(
            (
            text,    
            Node {
                align_content: AlignContent::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            }
        ));

        match e_type.0 {
            AppState::GameOver => next_state.set(AppState::GameOver),
            AppState::Victory => {
                next_state.set(AppState::Victory);
                difficulty.level += 1;
            },
            _ => next_state.set(AppState::Playing),
        }
    }
}

fn check_wave_clear(
    alien_query: Query<Entity, With<Alien>>,
    mut mission_ew: EventWriter<ChangeMissionEvent>,
    mut sent: Local<bool>, // Local memory for this system
) {
    if alien_query.is_empty() && !*sent {
        mission_ew.send(ChangeMissionEvent(MissionResult::Advance));
        *sent = true;
        
    } else if !alien_query.is_empty() {
        *sent = false;
    }
}

fn toggle_mission_result(
    mut mission_ev: EventReader<ChangeMissionEvent>,
    mut commands: Commands,
    mut difficulty: ResMut<Difficulty>,
    asset_server: Res<AssetServer>, 
    window: Single<&Window>,
    cannon_query: Query<Entity, With<Cannon>>,
    alien_query: Query<Entity, With<Alien>>,
    ball_query: Query<Entity, With<Ball>>,
) {
    for ev in mission_ev.read() {
        match ev.0 {
            MissionResult::Advance => { 
                difficulty.level += 1; 
                commands.insert_resource(Difficulty{level: difficulty.level});
            } 
            MissionResult::Restart => { difficulty.level = 1; }
        }

        for entity in cannon_query.iter() {
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).try_despawn();
            }
        }

        for entity in alien_query.iter() {
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).try_despawn();
            }
        }

        for entity in ball_query.iter() {
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).try_despawn();
            }
        }

        render_sprites(&mut commands, &asset_server, difficulty.level, &window);
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
        .insert_resource(Difficulty{level: 1})
        .insert_resource(AlienMoveDirection {
            dir: Direction::Right,
        })
        .insert_resource(AlienWaitToShoot {
            secs: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        })
        .add_event::<HudEvent>()
        .add_event::<ChangeMissionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_inputs,
                refresh_aliens,
                aliens_attack,
                refresh_balls,
                check_collisions,
                toggle_mission_result.after(check_collisions),
                check_wave_clear.after(toggle_mission_result),
                refresh_hud,
            ).run_if(in_state(AppState::Playing)),
        )
        .insert_state(AppState::Playing)
        .run();

}
