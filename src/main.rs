pub mod util;
use util::load_ron;
use fen::{ BoardState, Piece, PieceKind };

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

#[repr(usize)]
enum Space {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(Resource)]
struct Board(BoardState);

impl Board {
    pub fn move_piece(&mut self, original_space: Space, new_space: Space) {
        let space = original_space as usize;
        let og = &self.0.pieces[space.clone()];
        self.0.pieces[new_space as usize] = og.clone();
        self.0.pieces[space] = None;
    }
}

fn main() {

    let config = load_ron::<Config>("settings.ron");

    // let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";
    let board = Board(BoardState::from_fen(fen).unwrap());

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
                .insert_resource(board)
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

fn board_position(position: (usize, usize), config: &Res<Config>) -> Vec3 {
    let xf = ((position.0 as f32) - 4.0) * config.space_size;
    let yf = ((position.1 as f32) - 4.0) * config.space_size;
    Vec3::new(xf, yf, 0.0)
}

fn piece_index(piece: &Piece) -> usize {
    let index = match piece.kind {
        PieceKind::King => 0,
        PieceKind::Queen => 1,
        PieceKind::Bishop => 2,
        PieceKind::Knight => 3,
        PieceKind::Rook => 4,
        PieceKind::Pawn => 5,
    };
    let row = match piece.color {
        fen::Color::White => 0,
        fen::Color::Black => 6,
    };
    index + row
}

fn load_pieces(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut board: ResMut<Board>,
    ) {
    // Load pieces
    let texture_handle = asset_server.load("textures/chess_pieces.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(2560. / 6., 853. / 2.), 6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (i, opt) in board.0.pieces.iter().enumerate() {
        if let Some(piece) = opt {
            let position = (i % 8, i / 8);
            let index = piece_index(piece);
            commands.spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: index,
                    custom_size: Some(Vec2::new(config.space_size, config.space_size)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_translation(board_position(position, &config)),
                ..default()
            }));
        }
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
                        ColorMaterial::from(config.dark_color)
                    } else {
                        ColorMaterial::from(config.light_color)
                    }
                }),
                transform: Transform::from_translation(Vec3::new(((i as f32) - 3.5) * config.space_size, ((j as f32) - 3.5) * config.space_size, 0.)),
                ..default()
            });
        }
    }

}
