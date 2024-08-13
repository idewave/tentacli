use anyhow::{Result as AnyResult};
use std::io::BufRead;
use serde::{Serialize, Serializer};
use crate::BinaryConverter;

#[non_exhaustive]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WeatherState(pub u32);

#[allow(dead_code)]
impl WeatherState {
    pub const FINE: u32 = 0;
    pub const LIGHT_RAIN: u32 = 3;
    pub const MEDIUM_RAIN: u32 = 4;
    pub const HEAVY_RAIN: u32 = 5;
    pub const LIGHT_SNOW: u32 = 6;
    pub const MEDIUM_SNOW: u32 = 7;
    pub const HEAVY_SNOW: u32 = 8;
    pub const LIGHT_SANDSTORM: u32 = 22;
    pub const MEDIUM_SANDSTORM: u32 = 41;
    pub const HEAVY_SANDSTORM: u32 = 42;
    pub const THUNDERS: u32 = 86;
    pub const BLACKRAIN: u32 = 90;
}

impl BinaryConverter for WeatherState {
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        u32::write_into(&mut self.0, buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, dependencies: &mut Vec<u8>) -> AnyResult<Self> {
        let value = u32::read_from(reader, dependencies)?;

        Ok(Self(value))
    }
}

impl Serialize for WeatherState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let field_name = match self.0 {
            Self::FINE => "FINE",
            Self::LIGHT_RAIN => "LIGHT_RAIN",
            Self::MEDIUM_RAIN => "MEDIUM_RAIN",
            Self::HEAVY_RAIN => "HEAVY_RAIN",
            Self::LIGHT_SNOW => "LIGHT_SNOW",
            Self::MEDIUM_SNOW => "MEDIUM_SNOW",
            Self::HEAVY_SNOW => "HEAVY_SNOW",
            Self::LIGHT_SANDSTORM => "LIGHT_SANDSTORM",
            Self::MEDIUM_SANDSTORM => "MEDIUM_SANDSTORM",
            Self::HEAVY_SANDSTORM => "HEAVY_SANDSTORM",
            Self::THUNDERS => "THUNDERS",
            Self::BLACKRAIN => "BLACKRAIN",
            _ => "NONE",
        };

        serializer.serialize_str(field_name)
    }
}