use serde::{ser::Serializer, Serialize};

#[derive(Debug, thiserror::Error)]
/// Error transfered via Tauri API
pub enum Error {
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
    #[error("Not connected to {0}")]
    NotConnected(String),
    #[error(transparent)]
    IO(#[from] std::io::Error)
}

/// Error type shortcut
pub type Result<T> = std::result::Result<T, Error>;

/// Serialize error for readability
impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
