use super::asset::*;
use super::component::*;
use super::entity::*;
use super::resource::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::Anchor;

pub fn create_board_bundles() -> Vec<BoardEntity> {
    let mut bundles = Vec::new();
    let mut flag = true;
    for row in 0..8 {
        for col in 0..8 {
            bundles.push(BoardEntity {
                board: Board {},
                color: ChessColor {
                    kind: match flag {
                        true => ChessColorKind::Black,
                        false => ChessColorKind::White,
                    },
                },
                des: Description {
                    name: format!("{}_{}", row, col),
                    des: "board".to_string(),
                    help: "board".to_string(),
                },
                position: Position { row, col },
                theme: Theme {
                    asset_father_path: "default".to_string(),
                },
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                ..Default::default()
            });
            flag = !flag;
        }
        flag = !flag;
    }
    bundles
}

pub fn startup_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    assets: Res<AssetServer>,
) {
    game_state.current_turn = ChessColorKind::White;
    commands.spawn((
        Name::new("Camera"),
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.2,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(450.0, 450.0, 0.0),
    ));
    game_state.game_setting_handle = assets.load("default.setting.ron");
    game_state.game_setting_has_load = true;
    game_state.pieces_infos_handle = assets.load("default.pieces.ron");
    commands.spawn_batch(create_board_bundles());
}

pub fn create_pieces_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    infos: Res<Assets<PiecesInfos>>,
) {
    if game_state.pieces_infos_has_load {
        return;
    }
    let pieces_infos_option = infos.get(&game_state.pieces_infos_handle);

    if pieces_infos_option.is_none() {
        return;
    }

    let pieces_infos = pieces_infos_option.unwrap();

    let mut bundles = Vec::new();
    for info in &pieces_infos.pieces_info_vec {
        bundles.push(PiecesEntity {
            pieces: Pieces {
                kind: info.kind.clone(),
                ..Default::default()
            },
            color: ChessColor {
                kind: info.color.clone(),
            },
            des: info.des.clone(),
            position: Position {
                row: info.row,
                col: info.col,
            },
            theme: Theme {
                asset_father_path: info.theme.clone(),
            },
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            ..Default::default()
        });
    }
    commands.spawn_batch(bundles);
    game_state.pieces_infos_has_load = true;
}

pub fn show_pieces_system(
    mut query: Query<(
        &Pieces,
        &ChessColor,
        &Theme,
        &Position,
        &mut Transform,
        &mut Sprite,
        &mut Visibility,
    )>,
    asset_server: Res<AssetServer>,
) {
    for (pieces, color, theme, position, mut transform, mut texture, mut visibility) in &mut query {
        if color.kind == ChessColorKind::Gray {
            *visibility = Visibility::Hidden;
        } else {
            transform.translation.x = position.col as f32 * 128.0;
            transform.translation.y = position.row as f32 * 128.0;
            texture.image = asset_server.load(format!(
                "{}/{}/{}.png",
                theme.asset_father_path, color.kind, pieces.kind
            ));
        }
    }
}

pub fn show_board_system(
    mut query: Query<(
        &Board,
        &ChessColor,
        &Theme,
        &Position,
        &mut Transform,
        &mut Sprite,
    )>,
    asset_server: Res<AssetServer>,
) {
    for (_board, color, theme, position, mut transform, mut texture) in &mut query {
        transform.translation.x = position.col as f32 * 128.0;
        transform.translation.y = position.row as f32 * 128.0;
        texture.image = asset_server.load(format!(
            "{}/{}/board.png",
            theme.asset_father_path, color.kind
        ));
    }
}

pub fn select_pieces_system(
    mut query: Query<(&mut Pieces, &Position)>,
    game_state: ResMut<GameState>,
) {
    if let Some(selected_position) = game_state.selected_position.clone() {
        for (mut pieces, position) in &mut query {
            if selected_position == *position {
                pieces.selected = true;
                // info!("select {},{}",position.row,position.col);
            } else {
                pieces.selected = false;
            }
        }
    }
}

pub fn move_pieces_system(
    mut query: Query<(&mut Pieces, &mut Position, &mut ChessColor)>,
    mut game_state: ResMut<GameState>,
) {
    if let Some(move_position) = game_state.move_position.clone() {
        // is any other pieces
        let mut select_color = None;
        for (pieces, _position, color) in &mut query {
            if pieces.selected {
                select_color = Some(color.kind.clone());
            }
        }
        if let Some(select_color) = select_color {
            for (_pieces, position, mut color) in &mut query {
                if position.row == move_position.row && position.col == move_position.col {
                    if color.kind == select_color {
                        game_state.move_position = None;
                        return;
                    } else {
                        color.kind = ChessColorKind::Gray;
                    }
                }
            }
        }
        for (mut pieces, mut position, color) in &mut query {
            if pieces.selected && color.kind == game_state.current_turn {
                position.row = move_position.row;
                position.col = move_position.col;
                pieces.selected = false;
                if game_state.current_turn == ChessColorKind::White {
                    game_state.current_turn = ChessColorKind::Black;
                } else {
                    if game_state.current_turn == ChessColorKind::Black {
                        game_state.current_turn = ChessColorKind::White;
                    }
                }
            }
        }
        game_state.move_position = None;
    }
}
