use bevy::{prelude::*, utils::Instant, window::PrimaryWindow};

use crate::app::{self, Upgrades};
use crate::config::{CLICK_TEXT_DURATION, CLICK_TEXT_SPEED, CRIT_SIZE_MULTIPLIER};

use super::app::{Currency, Stats};
use super::config::{CLEAR_COLOR, CLICK_COLOR, CLICK_COLOR_CRIT};

#[derive(Component)]
struct Egg;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct ClickText {
    age: Instant,
}

impl Default for ClickText {
    fn default() -> Self {
        ClickText {
            age: Instant::now(),
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .add_startup_system(startup_system)
            .add_system(click_text_update_system)
            .add_system(click_system);
    }
}

/***********/
/* Systems */
/***********/

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    spawn_egg(&mut commands, &asset_server);
    spawn_score(&mut commands, &asset_server);

    spawn_upgrades_ui(&mut commands, &asset_server);
}

fn click_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut currency: ResMut<Currency>,
    stats: Res<Stats>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    egg_query: Query<&Transform, With<Egg>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // If click wasn't pressed, return
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    if let Some(cursor_position) =
        get_cursor_world_position(window_query.single(), camera, camera_transform)
    {
        let egg_transform = egg_query.single();
        //TODO implement scale stuff
        let scale = Vec3::new(120.0, 160.0, 1.0);
        if is_cursor_on_egg(&cursor_position, &egg_transform.translation, &scale) {
            let mut click_multiplier = stats.per_click.clone();

            let click_color;
            let crit = app::does_crit(stats.crit_chance);
            if crit {
                click_color = CLICK_COLOR_CRIT;
                click_multiplier = app::apply_crit(&click_multiplier, &stats.crit_mult);
            } else {
                click_color = CLICK_COLOR;
            }
            let click_multiplier_text = format!("+{}", click_multiplier);

            currency.0 += click_multiplier.clone();
            let mut text = text_query.single_mut();
            text.sections[0].value = currency.0.to_string();

            // Spawn floating +xxx text element
            commands
                .spawn(Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(
                        cursor_position.x,
                        cursor_position.y - 15.0,
                        5.0,
                    )),
                    text: Text::from_section(
                        click_multiplier_text.clone(),
                        TextStyle {
                            font: asset_server.load("fonts/press-start.regular.ttf"),
                            font_size: 20.0 * if crit { CRIT_SIZE_MULTIPLIER } else { 1.0 },
                            color: click_color,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                })
                .insert(ClickText::default());
        }
    }
}

fn click_text_update_system(
    mut query: Query<(&mut Transform, &ClickText, Entity)>,
    mut commands: Commands,
) {
    for (mut transform, clicked, entity) in query.iter_mut() {
        if Instant::now().duration_since(clicked.age) > CLICK_TEXT_DURATION {
            commands.entity(entity).despawn_recursive();
        } else {
            transform.translation.y += CLICK_TEXT_SPEED;
        }
    }
}

fn upgrade_system_button(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    upgrades: ResMut<Upgrades>,
) {
    for (interaction, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println!("Chicken");
            }
            _ => {}
        }
    }
}

/*************/
/* Functions */
/*************/

fn spawn_egg(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("textures/egg.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(160.0, 160.0)), // TODO scale var
                ..default()
            },
            ..default()
        })
        .insert(Egg)
        .id()
}

fn spawn_score(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn(Text2dBundle {
            transform: Transform::from_xyz(0.0, 250.0, 1.0),
            text: Text::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/press-start.regular.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        })
        .insert(ScoreText)
        .id()
}

fn spawn_upgrades_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(70.0),
                    ..default()
                },
                align_items: AlignItems::Start,
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                padding: UiRect {
                    left: Val::Percent(1.0),
                    right: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    bottom: Val::Percent(1.0),
                },
                gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            // Chicken Element
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        padding: UiRect {
                            left: Val::Percent(2.0),
                            right: Val::Percent(2.0),
                            top: Val::Percent(1.0),
                            bottom: Val::Percent(1.0),
                        },
                        gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(100.0), Val::Px(72.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(asset_server.load("textures/chicken.png")),
                        style: Style {
                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                            ..default()
                        },
                        ..default()
                    });
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "(0) Chicken\n\n+1 Eggs per second",
                            TextStyle {
                                font: asset_server.load("fonts/press-start.regular.ttf"),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Left),
                        ..default()
                    });
                });
        });
}

fn spawn_upgrade_element() {}

fn get_cursor_world_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    return window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
        .map(|pos2| Some(Vec3::new(pos2.x, pos2.y, 1.0)))
        .unwrap_or(None);
}

fn is_cursor_on_egg(cursor_translation: &Vec3, egg_translation: &Vec3, scale: &Vec3) -> bool {
    ((cursor_translation.x + egg_translation.x).abs() < (scale.x / 2.0))
        && ((cursor_translation.y + egg_translation.y).abs() < (scale.y / 2.0))
}
