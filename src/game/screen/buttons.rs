use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Buttons {
    Start,
    Exit,
    Settings,
    Other(String),
}
