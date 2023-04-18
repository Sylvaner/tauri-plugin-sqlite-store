use serde::{Deserialize};

#[derive(Deserialize)]
/// Option executed on database opening
pub struct OpenOptions {
    pub disable_foreign_keys: Option<bool>
}