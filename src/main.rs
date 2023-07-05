pub mod util;
use util::load_ron;

use serde::Deserialize;
use bevy::{
    prelude::*, 
    sprite::{
        MaterialMesh2dBundle, 
        Anchor,
    },
    render::color::Color,
    window::WindowResolution,
};

#[derive(Resource, Deserialize)]
struct Config {
    space_size: f32,
    light_color: Color,
    dark_color: Color
}

fn main() {

    let config = load_ron::<Config>("settings.ron");

    match config {
        Ok(c) => {
            App::new()
                .add_plugins(DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(Window {
                            resolution: (c.space_size * 8.0, c.space_size * 8.0).into(),
                            title: "Chessboard".into(),
                            ..default()
                        }),
                        ..default()
                    }
                ))
                .add_startup_system(setup)
                .insert_resource(c)
                .run();
            }
        Err(e) => {
            println!("{e}");
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {

    commands.spawn(Camera2dBundle::default());

    // Draw board
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(config.space_size, config.space_size)).into()).into(),
                material: materials.add({
                    if (i + j) % 2 == 0 {
                        ColorMaterial::from(config.light_color)
                    } else {
                        ColorMaterial::from(config.dark_color)
                    }
                }),
                transform: Transform::from_translation(Vec3::new(((i as f32) - 3.5) * config.space_size, ((j as f32) - 3.5) * config.space_size, 0.)),
                ..default()
            });
        }
    }

    // Load pieces
    let texture_handle = asset_server.load("textures/chess_pieces.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(2560. / 6., 853. / 2.), 6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite {
            index: 0,
            custom_size: Some(Vec2::new(config.space_size, config.space_size)),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        ..default()
    });
}
