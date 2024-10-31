use std::{
    io,
    path::{Path, PathBuf},
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to open file '{path}': {error}")]
    OpenFileError { path: PathBuf, error: io::Error },

    #[error("Unable to read directory '{path}': {error}")]
    ReadDirError { path: PathBuf, error: io::Error },

    #[error("Unable to resolve path '{path}': {error}")]
    ResolvePathError { path: PathBuf, error: io::Error },

    #[error("Expect the name of the file or directory '{0}' to be UTF-8 encoded")]
    ExpectUtf8FileName(PathBuf),

    #[error("Fail to parse JSON file '{path}': {error}")]
    ParseJsonFileError {
        error: serde_json::Error,
        path: PathBuf,
    },

    #[error("Expect a single media folder within '{0}', but found zero or multiple directories.")]
    ExpectSingleMediaDirectory(PathBuf),

    #[error("unable to decode data")]
    DecodeDataError,
}

impl Error {
    pub fn open_file_error<P>(path: P, error: io::Error) -> Self
    where
        P: AsRef<Path>,
    {
        Self::OpenFileError {
            path: path.as_ref().to_path_buf(),
            error,
        }
    }

    pub fn read_dir_error<P>(dir: P, error: io::Error) -> Self
    where
        P: AsRef<Path>,
    {
        Self::ReadDirError {
            path: dir.as_ref().to_path_buf(),
            error,
        }
    }

    pub fn resolve_path_error<P>(path: P, error: io::Error) -> Self
    where
        P: AsRef<Path>,
    {
        Self::ResolvePathError {
            path: path.as_ref().to_path_buf(),
            error,
        }
    }

    pub fn parse_json_file_error<P>(path: P, error: serde_json::Error) -> Self
    where
        P: AsRef<Path>,
    {
        Self::ParseJsonFileError {
            path: path.as_ref().to_path_buf(),
            error,
        }
    }

    pub fn expect_single_media_folder<P>(dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::ExpectSingleMediaDirectory(dir.as_ref().to_path_buf())
    }

    pub fn expect_utf8_file_name<P>(dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::ExpectUtf8FileName(dir.as_ref().to_path_buf())
    }
}
