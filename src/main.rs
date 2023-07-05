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
                .add_startup_system(load_pieces)
                .insert_resource(c)
                .run();
            }
        Err(e) => {
            println!("{e}");
        }
    }
}

fn board_position(position: (u8, u8), config: &Res<Config>) -> Vec3 {
    let xf = ((position.0 as f32) - 4.0) * config.space_size;
    let yf = ((position.1 as f32) - 4.0) * config.space_size;
    Vec3::new(xf, yf, 0.0)
}

#[derive(Component)]
struct Piece; 

fn load_pieces(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    ) {
    // Load pieces
    let texture_handle = asset_server.load("textures/chess_pieces.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(2560. / 6., 853. / 2.), 6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let pieces = vec![
        0, 1, 2, 2, 3, 3, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5,
        6, 7, 8, 8, 9, 9, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11
    ];
    let positions = vec![
        (3, 0), (4, 0), (2, 0), (5, 0), (1, 0), (6, 0), (0, 0), (7, 0),
        (3, 1), (4, 1), (2, 1), (5, 1), (1, 1), (6, 1), (0, 1), (7, 1),
        (3, 7), (4, 7), (2, 7), (5, 7), (1, 7), (6, 7), (0, 7), (7, 7),
        (3, 6), (4, 6), (2, 6), (5, 6), (1, 6), (6, 6), (0, 6), (7, 6),
    ];
    
    for (piece, position) in pieces.iter().zip(positions.iter()) {
        commands.spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: *piece,
                custom_size: Some(Vec2::new(config.space_size, config.space_size)),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(board_position(*position, &config)),
            ..default()
        }, Piece));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Config>,
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

}
