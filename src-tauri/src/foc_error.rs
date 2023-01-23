use tantivy::{directory::error::OpenReadError, query::QueryParserError};
pub type Result<T, E = FocError> = core::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum FocError {
    #[error("Metadata error: \"{0}\"")]
    Metadata(String),
    #[error("Filesystem error: \"{0}\"")]
    Fs(String),
    #[error("Database error: \"{0}\"")]
    Database(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
    #[error(transparent)]
    TantivyError(#[from] tantivy::TantivyError),
    #[error(transparent)]
    QueryParserError(#[from] QueryParserError),
    #[error(transparent)]
    OpenReadError(#[from] OpenReadError),
    #[error("Web extension error: \"{0}\"")]
    Extension(String),
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    ThumbError(#[from] thumbnailer::error::ThumbError),
    #[error(transparent)]
    DbPoolError(#[from] r2d2::Error),
    #[error(transparent)]
    DbMigrationError(#[from] diesel_migrations::MigrationError),
    #[error(transparent)]
    DieselDatabaseError(#[from] diesel::result::Error),
    #[error("Folder error: \"{0}\"")]
    Folder(String),
}

impl FocError {
    #[must_use]
    pub const fn error_code(&self) -> i32 {
        match self {
            FocError::Metadata(_) => 0,
            FocError::Io(_) => 1,
            FocError::Fs(_) => 2,
            FocError::Database(_) => 3,
            FocError::Serialization(_) => 4,
            FocError::TantivyError(_) => 5,
            FocError::QueryParserError(_) => 6,
            FocError::OpenReadError(_) => 7,
            FocError::Extension(_) => 8,
            FocError::DecodeError(_) => 9,
            FocError::ImageError(_) => 10,
            FocError::ThumbError(_) => 11,
            FocError::DbPoolError(_) => 12,
            FocError::DbMigrationError(_) => 13,
            FocError::DieselDatabaseError(_) => 14,
            FocError::Folder(_) => 15,
        }
    }
}

impl serde::Serialize for FocError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("code", &self.error_code())?;
        state.serialize_field("description", &self.to_string())?;
        state.end()
    }
}
