use std::collections::BTreeMap;
use crate::client::movement::parsers::types::Position;
use crate::types::AIManagerInput;

pub struct AI {
    distance_map: BTreeMap<u64, u32>,
}

impl AI {
    pub fn new() -> Self {
        Self {
            distance_map: BTreeMap::new(),
        }
    }

    pub async fn manage(&mut self, _input: AIManagerInput) {
        // ...
    }

    fn calculate_distance(from: Position, to: Position) -> f32 {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let dz = to.z - from.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}