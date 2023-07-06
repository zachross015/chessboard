pub mod util;
pub mod board;

use std::time::Duration;

use util::load_ron;
use board::{Board, Piece, PieceKind, Space, FromPrimitive};

use serde::Deserialize;
use bevy::{
    prelude::*, 
    sprite::{
        MaterialMesh2dBundle, 
        Anchor,
    },
    render::color::Color,
};
use bevy_tweening::*;
use bevy_tweening::lens::TransformPositionLens;

#[derive(Resource, Deserialize)]
struct Settings {
    space_size: f32,
    light_color: Color,
    dark_color: Color,
    piece_move_speed: u64,
}

fn main() {

    let config = load_ron::<Settings>("settings.ron");

    // let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";
    let board = Board::from_fen(fen).unwrap();

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
                .add_plugin(TweeningPlugin)
                .insert_resource(board)
                .add_startup_system(setup)
                .add_startup_system(load_pieces)
                .add_system(move_piece)
                .add_system(animated_move_piece)
                .insert_resource(c)
                .run();
            }
        Err(e) => {
            println!("{e}");
        }
    }
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
        board::Color::White => 0,
        board::Color::Black => 6,
    };
    index + row
}

fn load_pieces(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    config: Res<Settings>,
    board: Res<Board>,
    ) {
    // Load pieces
    let texture_handle = asset_server.load("textures/chess_pieces.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(2560. / 6., 853. / 2.), 6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (i, opt) in board.state().pieces.iter().enumerate() {
        if let Some(piece) = opt {
            let position = Space::from_usize(i).unwrap();
            let index = piece_index(piece);
            commands.spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index,
                    custom_size: Some(Vec2::new(config.space_size, config.space_size)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_translation(position.physical_position() * config.space_size),
                ..default()
            }, position));
        }
    }

}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Settings>,
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

    commands.spawn(AnimatedMovePiece(Space::A1, Space::A8));
    commands.spawn(AnimatedMovePiece(Space::B1, Space::B8));
    commands.spawn(AnimatedMovePiece(Space::C1, Space::C8));
    commands.spawn(AnimatedMovePiece(Space::D1, Space::D8));
    commands.spawn(AnimatedMovePiece(Space::E1, Space::E8));
    commands.spawn(AnimatedMovePiece(Space::F1, Space::F8));
    commands.spawn(AnimatedMovePiece(Space::G1, Space::G8));
    commands.spawn(AnimatedMovePiece(Space::H1, Space::H8));

}

// TODO Move all of these under an enum
#[derive(Component)]
struct MovePiece(Space, Space);

#[derive(Component)]
struct AnimatedMovePiece(Space, Space);

#[derive(Component)]
struct RemovePiece(Space);


fn move_piece(
    mut commands: Commands,
    mut board: ResMut<Board>,
    moves: Query<(Entity, &MovePiece)>,
    mut pieces: Query<(Entity, &mut Transform, &Space)>,
    config: Res<Settings>,
    ) {

    for (entity, m) in moves.iter() {
        board.move_piece(m.0, m.1);
        commands.entity(entity).despawn();

        for (piece_entity, mut transform, space) in &mut pieces {
            if *space == m.0 {
                transform.translation = m.1.physical_position() * config.space_size;
            } else if *space == m.1 {
                commands.entity(piece_entity).despawn();
            }
        }
    }
 
}

fn animated_move_piece(
    mut commands: Commands,
    mut board: ResMut<Board>,
    moves: Query<(Entity, &AnimatedMovePiece)>,
    pieces: Query<(Entity, &Transform, &Space)>,
    config: Res<Settings>,
    ) {

    for (entity, m) in moves.iter() {
        board.move_piece(m.0, m.1);
        commands.entity(entity).despawn();

        for (piece_entity, transform, space) in &pieces {
            if *space == m.0 {

                let tween = Tween::new(
                    EaseMethod::Linear,
                    Duration::from_millis(config.piece_move_speed),
                    TransformPositionLens {
                        start: transform.translation,
                        end: m.1.physical_position() * config.space_size,
                    }
                );

                commands.entity(piece_entity).insert(Animator::new(tween));
            } else if *space == m.1 {
                commands.entity(piece_entity).despawn();
            }
        }
    }
 
}
