use std::collections::HashMap;
use std::io::{Read, Seek};
use std::mem::size_of;
use std::sync::Arc;
use async_trait::async_trait;
use binrw::{BinRead, BinResult, Endian};
use serde::Serialize;
use tokio::sync::RwLock;

use crate::client::prelude::*;
use crate::enum_field;
use crate::plugins::wow::wotlk::realm::object::types::movement::{Movement, Point3D, SplineFlags};
use crate::plugins::wow::wotlk::realm::object::types::packed_guid::PackedGuid;
use crate::plugins::wow::wotlk::realm::object::update_object::Object;

#[derive(Packet, BinRead, Serialize, FieldsMetadata)]
#[br(little)]
struct Incoming {
    guid: PackedGuid,
    placeholder: u8,
    start_point: Point3D,
    spline_id: u32,

    move_type: MonsterMoveType,

    #[br(if(move_type == MonsterMoveType::FacingTarget))]
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<u64>,

    #[br(if(move_type == MonsterMoveType::FacingAngle))]
    #[serde(skip_serializing_if = "Option::is_none")]
    angle: Option<f32>,

    #[br(if(move_type == MonsterMoveType::FacingSpot))]
    #[serde(skip_serializing_if = "Option::is_none")]
    facing_point: Option<Point3D>,

    #[br(if(move_type != MonsterMoveType::Stop))]
    #[serde(skip_serializing_if = "Option::is_none")]
    spline_flags: Option<SplineFlags>,

    #[br(if(
            move_type != MonsterMoveType::Stop
            && spline_flags
                .as_ref()
                .is_some_and(|f| f.contains(SplineFlags::ANIMATION))
        ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_id: Option<u8>,

    #[br(if(
            move_type != MonsterMoveType::Stop
            && spline_flags
                .as_ref()
                .is_some_and(|f| f.contains(SplineFlags::ANIMATION))
        ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    parabolic_start_time: Option<i32>,

    #[br(if(move_type != MonsterMoveType::Stop))]
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,

    #[br(if(
            move_type != MonsterMoveType::Stop
            && spline_flags
                .as_ref()
                .is_some_and(|f| f.contains(SplineFlags::PARABOLIC))
        ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_acceleration: Option<f32>,

    #[br(if(
            move_type != MonsterMoveType::Stop
            && spline_flags
                .as_ref()
                .is_some_and(|f| f.contains(SplineFlags::PARABOLIC))
        ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_start_time: Option<i32>,
    #[br(if(move_type != MonsterMoveType::Stop))]
    path_size: u32,

    #[br(if(
            move_type != MonsterMoveType::Stop
            && spline_flags
                .as_ref()
                .is_some_and(|f| !f.is_catmull_rom())
        ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_point: Option<Point3D>,

    #[br(if(
            move_type != MonsterMoveType::Stop
            && spline_flags
                .as_ref()
                .is_some_and(|f| !f.is_catmull_rom())
            && path_size > 1
        ))]
    // it seems the "count" attr is evaluated independently of "if" attr
    // so to avoid panic saturating_sub should be used here
    #[br(count = path_size.saturating_sub(1))]
    #[serde(skip_serializing_if = "Option::is_none")]
    linear_path: Option<Vec<LinearPoint3D>>,

    #[br(if(
            move_type != MonsterMoveType::Stop
            && spline_flags
                .as_ref()
                .is_some_and(|f| f.is_catmull_rom())
        ))]
    #[br(count = path_size)]
    #[serde(skip_serializing_if = "Option::is_none")]
    catmull_points: Option<Vec<Point3D>>,
}

pub struct Handler;

#[async_trait]
impl PacketHandler for Handler {
    async fn handle(
        &mut self,
        packet: &mut Packet,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<HandlerOutput>> {
        let mut outputs = vec![];
        let mut incoming = Incoming::unpack(packet)?;

        if let (
            Some(offsets),
            Some(dest),
        ) = (
            incoming.linear_path.take(),
            incoming.destination_point,
        ) {
            let start = incoming.start_point;

            let middle = Point3D {
                x: (start.x + dest.x) * 0.5,
                y: (start.y + dest.y) * 0.5,
                z: (start.z + dest.z) * 0.5,
            };

            let mut world_path = Vec::with_capacity(offsets.len() + 2);

            // First point (start)
            world_path.push(start);

            // Internal points (decoded from packed offsets)
            for lp in offsets {
                let off = lp.0;
                world_path.push(Point3D {
                    x: middle.x - off.x,
                    y: middle.y - off.y,
                    z: middle.z - off.z,
                });
            }

            // Last point (destination)
            world_path.push(dest);

            let linear_points: Vec<LinearPoint3D> =
                world_path.iter().map(|p| LinearPoint3D(*p)).collect();

            // Overwrite the original field with absolute world-space points
            incoming.linear_path = Some(linear_points.clone());

            packet.set_json(
                serialize_packet_json(&incoming)?
            );

            outputs.push(HandlerOutput::Requests(vec![
                Request::SetContext(Some(Box::new(move |ctx: &mut CtxMap| {
                    let Some(objects) =
                        ctx.get_mut::<HashMap<PackedGuid, Object>>() else {
                        return;
                    };

                    let Some(object) = objects.get_mut(&incoming.guid) else {
                        return;
                    };

                    let movement = object.movement.get_or_insert_with(|| Movement {
                        ..Default::default()
                    });

                    movement.spline_info
                        .get_or_insert_with(Default::default)
                        .path = world_path.clone();
                }))),
            ]))
        }

        Ok(outputs)
    }
}

enum_field! {
    enum MonsterMoveType: u8 {
        Normal = 0,
        Stop = 1,
        FacingSpot = 2,
        FacingTarget = 3,
        FacingAngle = 4,
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
#[serde(transparent)]
pub struct LinearPoint3D(pub Point3D);

impl BinRead for LinearPoint3D {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let packed: u32 = u32::read_options(reader, Endian::Little, ())?;

        // Extract raw fields
        let raw_x = (packed & 0x7FF) as i32;          // 11 bits
        let raw_y = ((packed >> 11) & 0x7FF) as i32;  // 11 bits
        let raw_z = ((packed >> 22) & 0x3FF) as i32;  // 10 bits

        // Sign-extend
        let sx = sign_extend(raw_x, 11);
        let sy = sign_extend(raw_y, 11);
        let sz = sign_extend(raw_z, 10);

        // Scale back to float (step = 0.25)
        let offset = Point3D {
            x: sx as f32 * 0.25,
            y: sy as f32 * 0.25,
            z: sz as f32 * 0.25,
        };

        Ok(LinearPoint3D(offset))
    }
}

impl CalculateMetadata for LinearPoint3D {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let size = size_of::<u32>();

        ctx.metadata.insert(
            ctx.current_key.clone(),
            MetadataValue {
                size,
                offset: ctx.offset,
            },
        );

        ctx.offset += size;
        ctx
    }
}

fn sign_extend(value: i32, bits: u32) -> i32 {
    let shift = 32 - bits;
    (value << shift) >> shift
}
