use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32, orientation: f32) -> Self {
        Self { x, y, z, orientation }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {:?}, y: {:?}, z: {:?}, orientation: {:?}",
            self.x,
            self.y,
            self.z,
            self.orientation,
        )
    }
}