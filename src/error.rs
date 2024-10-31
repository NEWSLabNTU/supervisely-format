use std::{io, path::PathBuf};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to open file '{path}': {error}")]
    OpenFileFailure { path: PathBuf, error: io::Error },

    #[error("Unable to read directory '{path}': {error}")]
    ReadDirFailure { path: PathBuf, error: io::Error },

    #[error("The name of the media media directory '{0}' is not UTF-8 encoded")]
    NonUtf8MediaName(PathBuf),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("No media folder found in directory '{0}'")]
    NoMediaFolderFound(PathBuf),

    #[error("Expect a single media folder, but found more than one in directory {0}")]
    MultipleMediaFoldersFound(PathBuf),
}
