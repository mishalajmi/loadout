use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadoutError {
    #[error("Could not find config directory")]
    ConfigDirNotFound,
    #[error("Failed to read config file at {path}: {source}")]
    ConfigReadError {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to parse config file: {0}")]
    ConfigParseError(#[from] serde_json::Error),
    #[error("Failed to write config file at {path}: {source}")]
    ConfigWriteError {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Failed to create config directory at {path}: {source}")]
    ConfigDirCreateError {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("No directory selected")]
    NoDirectorySelected,
    #[error("Invalid directory path: {0}")]
    InvalidDirectory(PathBuf),
}

pub type Result<T> = anyhow::Result<T, LoadoutError>;
