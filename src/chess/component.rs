use std::fmt::Display;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
#[derive(Component,Default,Clone, Serialize, Deserialize)]
pub struct Description {
    pub name:String,
    pub des:String,
    pub help:String,
}

#[derive(Default,Debug,Clone, Serialize, Deserialize)]
pub enum ChessColorKind {
    #[default]
    White,
    Black,
    Gray
}

impl Display for ChessColorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessColorKind::White => f.write_str("white"),
            ChessColorKind::Black => f.write_str("black"),
            ChessColorKind::Gray => f.write_str("gray"),
        }
    }
}



#[derive(Component,Default)]
pub struct ChessColor {
    pub kind:ChessColorKind
}

#[derive(Component,Default,Debug,PartialEq,Clone)]
pub struct Position {
    pub row:u8,
    pub col:u8,
}


#[derive(Default,Debug,Clone, Serialize, Deserialize)]
pub enum PiecesKind {
    #[default]
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl Display for PiecesKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PiecesKind::King => f.write_str("king"),
            PiecesKind::Queen => f.write_str("queen"),
            PiecesKind::Rook => f.write_str("rook"),
            PiecesKind::Knight => f.write_str("knight"),
            PiecesKind::Bishop => f.write_str("bishop"),
            PiecesKind::Pawn => f.write_str("pawn"),
        }
    }
}

#[derive(Component,Default,Debug)]
pub struct Pieces {
    pub kind:PiecesKind,
    pub selected:bool,
}

#[derive(Component,Default)]
pub struct Board {
    pub invalid:bool,
}
#[derive(Component,Default)]
pub struct Theme {
    pub asset_father_path:String,
}