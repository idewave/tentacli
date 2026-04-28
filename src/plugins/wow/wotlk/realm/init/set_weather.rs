use async_trait::async_trait;
use binrw::BinRead;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::enum_field;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    weather_state: WeatherState,
    weather_grade: f32,
    unknown: u8,
}

pub struct Handler;
#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let _ = Incoming::unpack(packet)?;
        Ok(vec![])
    }
}

enum_field! {
    pub enum WeatherState: u32 {
        Fine = 0,
        LightRain = 3,
        MediumRain = 4,
        HeavyRain = 5,
        LightSnow = 6,
        MediumSnow = 7,
        HeavySnow = 8,
        LightSandstorm = 22,
        MediumSandstorm = 41,
        HeavySandstorm = 42,
        Thunders = 86,
        BlackRain = 90,
    }
}

// #[repr(u32)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum WeatherState {
//     Fine = 0,
//     LightRain = 3,
//     MediumRain = 4,
//     HeavyRain = 5,
//     LightSnow = 6,
//     MediumSnow = 7,
//     HeavySnow = 8,
//     LightSandstorm = 22,
//     MediumSandstorm = 41,
//     HeavySandstorm = 42,
//     Thunders = 86,
//     BlackRain = 90,
//
//     Unknown(u32),
// }
//
// impl BinRead for WeatherState {
//     type Args<'a> = ();
//
//     fn read_options<R: Read + Seek>(
//         reader: &mut R,
//         endian: binrw::Endian,
//         _args: Self::Args<'_>,
//     ) -> BinResult<Self> {
//         let raw: u32 = u32::read_options(reader, endian, ())?;
//         Ok(match raw {
//             0  => WeatherState::Fine,
//             3  => WeatherState::LightRain,
//             4  => WeatherState::MediumRain,
//             5  => WeatherState::HeavyRain,
//             6  => WeatherState::LightSnow,
//             7  => WeatherState::MediumSnow,
//             8  => WeatherState::HeavySnow,
//             22 => WeatherState::LightSandstorm,
//             41 => WeatherState::MediumSandstorm,
//             42 => WeatherState::HeavySandstorm,
//             86 => WeatherState::Thunders,
//             90 => WeatherState::BlackRain,
//             other => WeatherState::Unknown(other),
//         })
//     }
// }
//
// impl CalculateMetadata for WeatherState {
//     fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
//         let size = size_of::<u32>();
//         ctx.metadata.insert(
//             ctx.current_key.clone(),
//             MetadataValue { size, offset: ctx.offset },
//         );
//         ctx.offset += size;
//         ctx
//     }
// }
//
// impl Serialize for WeatherState {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let field_name = match self {
//             Self::Fine => "FINE",
//             Self::LightRain => "LIGHT_RAIN",
//             Self::MediumRain => "MEDIUM_RAIN",
//             Self::HeavyRain => "HEAVY_RAIN",
//             Self::LightSnow => "LIGHT_SNOW",
//             Self::MediumSnow => "MEDIUM_SNOW",
//             Self::HeavySnow => "HEAVY_SNOW",
//             Self::LightSandstorm => "LIGHT_SANDSTORM",
//             Self::MediumSandstorm => "MEDIUM_SANDSTORM",
//             Self::HeavySandstorm => "HEAVY_SANDSTORM",
//             Self::Thunders => "THUNDERS",
//             Self::BlackRain => "BLACKRAIN",
//             _ => "NONE",
//         };
//
//         serializer.serialize_str(field_name)
//     }
// }
