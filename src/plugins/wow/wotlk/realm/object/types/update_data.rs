use bitflags::bitflags;
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use std::io::{Read, Seek, Write};

use bitflags_extras::BitflagExtras;
use crate::client::prelude::*;
use crate::plugins::wow::wotlk::realm::object::types::update_fields::{
    ContainerField, CorpseField, DynamicObjectField, FieldEnum, FieldValue,
    GameObjectField, ItemField, ObjectField, PlayerField, UnitField
};

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct UpdateData {
    #[serde(skip)]
    pub blocks_amount: u8,
    #[serde(skip)]
    pub raw_indices: Vec<u32>,
    #[serde(skip_serializing_if = "ObjectTypeMask::is_empty")]
    pub object_type_mask: ObjectTypeMask,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub object_fields: BTreeMap<ObjectField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub unit_fields: BTreeMap<UnitField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub player_fields: BTreeMap<PlayerField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub item_fields: BTreeMap<ItemField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub container_fields: BTreeMap<ContainerField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub game_object_fields: BTreeMap<GameObjectField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub dynamic_object_fields: BTreeMap<DynamicObjectField, FieldValue>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub corpse_fields: BTreeMap<CorpseField, FieldValue>,
}

impl UpdateData {
    #[inline]
    fn parse_value(option: &FieldValue) -> Vec<u32> {
        match option {
            FieldValue::Long(v) => {
                let hi = (v >> 32) as u32;
                let lo = (v & 0xFFFF_FFFF) as u32;
                vec![lo, hi]
            }

            FieldValue::LongArray(values) => {
                let mut out = Vec::new();
                for v in values.iter().flatten() {
                    let hi = (v >> 32) as u32;
                    let lo = (v & 0xFFFF_FFFF) as u32;
                    out.push(lo);
                    out.push(hi);
                }
                out
            }

            FieldValue::Integer(v) => vec![*v as u32],

            FieldValue::IntegerArray(vs) => {
                vs.iter()
                    .filter_map(|v| v.map(|x| x as u32))
                    .collect()
            }

            FieldValue::Bytes(v) => vec![*v],

            FieldValue::BytesArray(vs) => {
                vs.iter()
                    .filter_map(|v| *v)
                    .collect()
            }

            FieldValue::Float(v) => vec![v.to_bits()],

            FieldValue::FloatArray(vs) => {
                vs.iter()
                    .filter_map(|v| v.map(|f| f.to_bits()))
                    .collect()
            }

            FieldValue::TwoShorts((a, b)) => {
                vec![((*a as u32) << 16) | *b as u32]
            }

            FieldValue::TwoShortsArray(vs) => {
                vs.iter()
                    .filter_map(|v| {
                        v.map(|(a, b)| ((a as u32) << 16) | b as u32)
                    })
                    .collect()
            }

            _ => vec![],
        }
    }

    #[inline]
    fn build_blocks(
        update_blocks: &BTreeMap<u32, u32>, start: u32, end: u32
    ) -> BTreeMap<u32, u32> {
        update_blocks.range(start..=end).map(|(&k,&v)| (k,v)).collect()
    }

    #[inline]
    fn values_limit(&self) -> u32 {
        if self.object_type_mask.contains(ObjectTypeMask::PLAYER) {
            PlayerField::get_limit()
        } else if self.object_type_mask.contains(ObjectTypeMask::UNIT) {
            UnitField::get_limit()
        } else if self.object_type_mask.contains(ObjectTypeMask::ITEM) {
            ItemField::get_limit()
        } else if self.object_type_mask.contains(ObjectTypeMask::GAMEOBJECT) {
            GameObjectField::get_limit()
        } else if self.object_type_mask.contains(ObjectTypeMask::DYNAMICOBJECT) {
            DynamicObjectField::get_limit()
        } else if self.object_type_mask.contains(ObjectTypeMask::CORPSE) {
            CorpseField::get_limit()
        } else {
            ObjectField::get_limit()
        }
    }

    fn push_field_values<F: FieldEnum>(
        fields: &BTreeMap<F, FieldValue>,
        values_stream: &mut BTreeMap<u32, u32>,
    ) {
        for (field_key, field_value) in fields {
            let base_index = F::get_index(field_key);

            for (offset, value) in UpdateData::parse_value(field_value).into_iter().enumerate() {
                let global_slot_index = base_index + offset as u32;

                values_stream.insert(global_slot_index, value);
            }
        }
    }

    fn collect_update_fields(&self) -> (BTreeMap<u32, u32>, u32 /* values_limit */) {
        let mut values_stream: BTreeMap<u32, u32> = BTreeMap::new();

        Self::push_field_values(&self.object_fields, &mut values_stream);
        Self::push_field_values(&self.unit_fields, &mut values_stream);
        Self::push_field_values(&self.player_fields, &mut values_stream);
        Self::push_field_values(&self.item_fields, &mut values_stream);
        Self::push_field_values(&self.container_fields, &mut values_stream);
        Self::push_field_values(&self.game_object_fields, &mut values_stream);
        Self::push_field_values(&self.dynamic_object_fields, &mut values_stream);
        Self::push_field_values(&self.corpse_fields, &mut values_stream);

        let values_limit = self.values_limit();

        (values_stream, values_limit)
    }

    fn try_group<F: FieldEnum>(
        &self,
        fields: &BTreeMap<F, FieldValue>,
        group: &str,
        slot: u32,
        present_slots: &HashSet<u32>,
        ctx: &mut MetadataContext,
    ) -> Option<u32> {
        let variant = F::get_variant_by_index(slot)?;
        let field_value = fields.get(&variant)?;

        let layout_slots = F::get_layout_slots(&variant);

        // Count how many u32 slots are physically present in the stream
        let mut present_count = 0usize;
        for s in slot..(slot + layout_slots) {
            if present_slots.contains(&s) {
                present_count += 1;
            }
        }

        let base_offset = ctx.offset;
        let total_size = present_count * 4;

        let field_key = format!(
            "{}/{}/{}",
            ctx.current_key,
            group,
            variant.get_field_name()
        );

        // Metadata for the whole field
        ctx.metadata.insert(
            field_key.clone(),
            MetadataValue {
                size: total_size,
                offset: base_offset,
            },
        );

        // Element-level metadata
        if field_value.is_array_like() {
            match field_value {
                // ---- CustomArray = array of rows (struct-like) ----
                FieldValue::CustomArray(rows) => {
                    let mut stream_offset = base_offset;

                    for (logical_index, row) in rows.iter().enumerate() {
                        // Render this element if at least one slot in the row is present
                        let has_any = row.iter().any(|slot| slot.is_some());

                        // Physical size of this row in bytes
                        let row_size: usize = row
                            .iter()
                            .map(|slot| match slot {
                                Some(v) => v.len(),
                                None => 4, // Empty slot still occupies one u32 in the stream
                            })
                            .sum();

                        if has_any {
                            let element_key = format!("{}/{}", field_key, logical_index);

                            ctx.metadata.insert(
                                element_key,
                                MetadataValue {
                                    size: row_size,
                                    offset: stream_offset,
                                },
                            );
                        }

                        stream_offset += row_size;
                    }
                }

                // ---- All other array types (IntegerArray, LongArray, etc.) ----
                _ => {
                    let stride = field_value.element_stride_slots(); // u32 slots per logical element

                    let mut logical_index = 0usize;
                    let mut stream_index = 0usize;

                    let end_slot = slot + layout_slots;
                    let mut s = slot;

                    while s < end_slot {
                        // Check only the first slot of the logical element
                        let element_present = present_slots.contains(&s);

                        let should_render = match field_value {
                            FieldValue::IntegerArray(v) => {
                                v.get(logical_index).and_then(|x| *x).is_some()
                            }
                            FieldValue::FloatArray(v) => {
                                v.get(logical_index).and_then(|x| *x).is_some()
                            }
                            FieldValue::BytesArray(v) => {
                                v.get(logical_index).and_then(|x| *x).is_some()
                            }
                            FieldValue::TwoShortsArray(v) => {
                                v.get(logical_index).and_then(|x| *x).is_some()
                            }
                            FieldValue::LongArray(v) => {
                                v.get(logical_index).and_then(|x| *x).is_some()
                            }
                            _ => false,
                        };

                        if element_present && should_render {
                            let element_key = format!("{}/{}", field_key, logical_index);

                            ctx.metadata.insert(
                                element_key,
                                MetadataValue {
                                    size: stride * 4,
                                    offset: base_offset + stream_index * stride * 4,
                                },
                            );
                        }

                        // Advance physical stream index only if this element exists in the stream
                        if element_present {
                            stream_index += 1;
                        }

                        // Always advance by one logical element
                        logical_index += 1;
                        s += stride as u32;
                    }
                }
            }
        }

        ctx.offset += total_size;

        Some(layout_slots)
    }
}

impl BinRead for UpdateData {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>
    ) -> BinResult<Self> {
        let blocks_amount = u8::read_options(reader, endian, ())?;

        let mut out = Self::default();
        if blocks_amount == 0 {
            return Ok(out);
        }

        let mut mask_bits = Vec::with_capacity(blocks_amount as usize * 32);
        for _ in 0..blocks_amount {
            let mut mask = u32::read_options(reader, endian, ())?;
            for _ in 0..32 {
                mask_bits.push((mask & 1) == 1);
                mask >>= 1;
            }
        }

        let indices: Vec<u32> = mask_bits.iter()
            .enumerate()
            .filter_map(|(i, &value)| if value { Some(i as u32) } else { None })
            .collect();

        out.blocks_amount = blocks_amount;
        out.raw_indices = indices.clone();

        let mut update_blocks: BTreeMap<u32, u32> = BTreeMap::new();
        for idx in indices {
            let v = u32::read_options(reader, endian, ())?;
            update_blocks.insert(idx, v);
        }

        out.object_fields = {
            let blocks = UpdateData::build_blocks(&update_blocks, 0, ObjectField::get_limit());
            ObjectField::read_from(blocks).unwrap_or_default()
        };

        let object_type_mask = out.object_fields
            .get(&ObjectField::Type)
            .and_then(|field| {
                if let FieldValue::Integer(mask) = field {
                    Some(ObjectTypeMask::from_bits_truncate(*mask))
                } else {
                    None
                }
            })
            .unwrap_or(ObjectTypeMask::NONE);

        out.object_type_mask = object_type_mask;

        if object_type_mask.contains(ObjectTypeMask::UNIT) || object_type_mask.is_empty() {
            let blocks = UpdateData::build_blocks(
                &update_blocks, ObjectField::get_limit()+1, UnitField::get_limit()
            );
            out.unit_fields = UnitField::read_from(blocks).unwrap_or_default();
        }

        if object_type_mask.contains(ObjectTypeMask::PLAYER) || object_type_mask.is_empty() {
            let blocks = UpdateData::build_blocks(
                &update_blocks, UnitField::get_limit()+1, PlayerField::get_limit()
            );
            out.player_fields = PlayerField::read_from(blocks).unwrap_or_default();
        }

        if object_type_mask.contains(ObjectTypeMask::ITEM) || object_type_mask.is_empty() {
            let blocks = UpdateData::build_blocks(
                &update_blocks, ObjectField::get_limit()+1, ItemField::get_limit()
            );
            out.item_fields = ItemField::read_from(blocks).unwrap_or_default();
        }

        if object_type_mask.contains(ObjectTypeMask::GAMEOBJECT) || object_type_mask.is_empty() {
            let blocks = UpdateData::build_blocks(
                &update_blocks, ObjectField::get_limit()+1, GameObjectField::get_limit()
            );
            out.game_object_fields = GameObjectField::read_from(blocks).unwrap_or_default();
        }

        if object_type_mask.contains(ObjectTypeMask::DYNAMICOBJECT) || object_type_mask.is_empty() {
            let blocks = UpdateData::build_blocks(
                &update_blocks, ObjectField::get_limit()+1, DynamicObjectField::get_limit()
            );
            out.dynamic_object_fields = DynamicObjectField::read_from(blocks).unwrap_or_default();
        }

        if object_type_mask.contains(ObjectTypeMask::CONTAINER) || object_type_mask.is_empty() {
            let blocks = UpdateData::build_blocks(
                &update_blocks, ItemField::get_limit()+1, ContainerField::get_limit()
            );
            out.container_fields = ContainerField::read_from(blocks).unwrap_or_default();
        }

        if object_type_mask.contains(ObjectTypeMask::CORPSE) || object_type_mask.is_empty() {
            let blocks = UpdateData::build_blocks(
                &update_blocks, ObjectField::get_limit()+1, CorpseField::get_limit()
            );
            out.corpse_fields = CorpseField::read_from(blocks).unwrap_or_default();
        }

        Ok(out)
    }
}

impl BinWrite for UpdateData {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _: Self::Args<'_>
    ) -> BinResult<()> {
        let (update_fields, values_limit) = self.collect_update_fields();

        // blocks_amount
        let blocks_amount: u8 = ((values_limit + 31) / 32) as u8;
        u8::write_options(&blocks_amount, writer, endian, ())?;

        // mask
        let mut mask_vec: Vec<u32> = vec![0; blocks_amount as usize];
        for index in update_fields.keys() {
            let bi = (index / 32) as usize;
            let bit = index % 32;
            mask_vec[bi] |= 1 << bit;
        }
        for m in &mask_vec {
            u32::write_options(m, writer, endian, ())?;
        }

        // values
        for v in update_fields.values() {
            u32::write_options(v, writer, endian, ())?;
        }

        Ok(())
    }
}

impl CalculateMetadata for UpdateData {
    fn calculate<'a>(&self, ctx: &'a mut MetadataContext) -> &'a mut MetadataContext {
        let present_slots: HashSet<u32> = self.raw_indices.iter().copied().collect();
        // blocks_amount (u8)
        ctx.offset += 1;

        // mask
        ctx.offset += (self.blocks_amount as usize) * 4;

        let values_limit = self.values_limit();

        let mut slot = 0u32;
        while slot < values_limit {
            let consumed_opt = self.try_group::<ObjectField>(
                &self.object_fields, "object_fields", slot, &present_slots, ctx
            ).or_else(|| {
                self.try_group::<UnitField>(
                    &self.unit_fields, "unit_fields", slot, &present_slots, ctx
                )
            }).or_else(|| {
                self.try_group::<PlayerField>(
                    &self.player_fields, "player_fields", slot, &present_slots, ctx
                )
            }).or_else(|| {
                self.try_group::<ItemField>(
                    &self.item_fields, "item_fields", slot, &present_slots, ctx
                )
            }).or_else(|| {
                self.try_group::<ContainerField>(
                    &self.container_fields, "container_fields", slot, &present_slots, ctx
                )
            }).or_else(|| {
                self.try_group::<GameObjectField>(
                    &self.game_object_fields, "game_object_fields", slot, &present_slots, ctx
                )
            }).or_else(|| {
                self.try_group::<DynamicObjectField>(
                    &self.dynamic_object_fields, "dynamic_object_fields", slot, &present_slots, ctx
                )
            }).or_else(|| {
                self.try_group::<CorpseField>(
                    &self.corpse_fields, "corpse_fields", slot, &present_slots, ctx
                )
            });

            if let Some(consumed) = consumed_opt {
                slot += consumed;
            } else {
                if present_slots.contains(&slot) {
                    ctx.offset += 4;
                }
                slot += 1;
            }
        }

        ctx
    }
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, BitflagExtras)]
    #[bitflags_repr(i32)]
    pub struct ObjectTypeMask: i32 {
        const NONE = 0x0000;
        const OBJECT = 0x0001;
        const ITEM = 0x0002;
        const CONTAINER = 0x0004;
        const UNIT = 0x0008;
        const PLAYER = 0x0010;
        const GAMEOBJECT = 0x0020;
        const DYNAMICOBJECT = 0x0040;
        const CORPSE = 0x0080;
    }
}