#![deny(clippy::all)]

use serde::{Serialize, Deserialize};
use crate::{utils::Color, error::Error};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Theme {
    pub name: String,
    pub primary_color: Color,
    pub secondary_color: Color,
}

impl Theme {

    pub fn load() -> Result<Self, Error> {
        // TODO: This is currently a placeholder
        Ok(Self {
            name: String::new(),
            primary_color: Color::Gray,
            secondary_color: Color::Gray,
        })
    }

}