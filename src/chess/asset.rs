use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::{thiserror, BoxedFuture},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::component::{ChessColorKind, Description, PiecesKind};

#[derive(Asset, TypePath, Default, Serialize, Deserialize)]
pub struct PiecesInfos {
    pub pieces_info_vec: Vec<PiecesInfo>,
}

#[derive(Asset, TypePath, Default, Serialize, Deserialize)]
pub struct GameSetting {}

#[derive(Default, Serialize, Deserialize)]
pub struct PiecesInfo {
    pub des: Description,
    pub color: ChessColorKind,
    pub kind: PiecesKind,
    pub row: u8,
    pub col: u8,
    pub theme: String,
}

#[derive(Debug, Error)]
pub enum AssetLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    RonError(#[from] ron::error::Error),
    #[error(transparent)]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error(transparent)]
    LoadDirectError(#[from] bevy::asset::LoadDirectError),
}

#[derive(Default)]
pub struct GameSettingLoader;

impl AssetLoader for GameSettingLoader {
    type Asset = GameSetting;

    type Settings = ();

    type Error = AssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron = ron::de::from_bytes(&bytes)?;
            Ok(ron)
        })
    }
    fn extensions(&self) -> &[&str] {
        &["setting.ron"]
    }
}

#[derive(Default)]
pub struct PiecesInfosLoader;

impl AssetLoader for PiecesInfosLoader {
    type Asset = PiecesInfos;
    type Settings = ();
    type Error = AssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ron = ron::de::from_bytes(&bytes)?;
            Ok(ron)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["pieces.ron"]
    }
}
