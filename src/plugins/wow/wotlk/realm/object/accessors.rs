use crate::plugins::wow::wotlk::realm::object::types::movement::Point3D;
use crate::plugins::wow::wotlk::realm::object::types::unit::{Class, Gender, PowerType, Race};
use crate::plugins::wow::wotlk::realm::object::types::update_fields::{
    FieldValue, GameObjectField, ObjectField, PlayerField, UnitField,
};
use crate::plugins::wow::wotlk::realm::object::{Object, ObjectTypeId, PackedGuid};

#[derive(Clone, Copy)]
pub struct UnitRef<'a> {
    object: &'a Object,
}

#[derive(Clone, Copy)]
pub struct PlayerRef<'a> {
    object: &'a Object,
}

#[derive(Clone, Copy)]
pub struct GameObjectRef<'a> {
    object: &'a Object,
}

impl Object {
    #[inline]
    pub fn guid(&self) -> PackedGuid {
        self.guid
    }

    #[inline]
    pub fn object_type_id(&self) -> ObjectTypeId {
        self.object_type_id
    }

    #[inline]
    pub fn entry_id(&self) -> Option<u32> {
        field_u32(self.object_fields.get(&ObjectField::Entry))
    }

    #[inline]
    pub fn scale(&self) -> Option<f32> {
        field_f32(self.object_fields.get(&ObjectField::ScaleX))
    }

    /// Best currently known world-space position for this object.
    #[inline]
    pub fn position(&self) -> Option<Point3D> {
        let movement = self.movement.as_ref()?;

        if let Some(info) = movement.movement_info.as_ref() {
            return Some(info.location.point);
        }
        if let Some(info) = movement.position_info.as_ref() {
            return Some(info.location.point);
        }
        if let Some(position) = movement.world_object_position.as_ref() {
            return Some(position.point);
        }
        if let Some(position) = movement.game_object_position.as_ref() {
            return Some(position.point);
        }

        None
    }

    /// Best currently known facing/orientation in radians.
    #[inline]
    pub fn facing(&self) -> Option<f32> {
        let movement = self.movement.as_ref()?;

        if let Some(info) = movement.movement_info.as_ref() {
            return Some(info.location.direction);
        }
        if let Some(position) = movement.world_object_position.as_ref() {
            return Some(position.direction);
        }
        if let Some(position) = movement.game_object_position.as_ref() {
            return Some(position.direction);
        }
        if let Some(info) = movement.position_info.as_ref() {
            return Some(info.location.direction);
        }

        None
    }

    /// Packed game-object rotation supplied by the movement block, when present.
    #[inline]
    pub fn rotation(&self) -> Option<i64> {
        self.movement.as_ref()?.game_object_rotation
    }

    #[inline]
    pub fn as_unit(&self) -> Option<UnitRef<'_>> {
        matches!(self.object_type_id, ObjectTypeId::Unit | ObjectTypeId::Player)
            .then_some(UnitRef { object: self })
    }

    #[inline]
    pub fn as_player(&self) -> Option<PlayerRef<'_>> {
        matches!(self.object_type_id, ObjectTypeId::Player).then_some(PlayerRef { object: self })
    }

    #[inline]
    pub fn as_game_object(&self) -> Option<GameObjectRef<'_>> {
        matches!(self.object_type_id, ObjectTypeId::GameObject)
            .then_some(GameObjectRef { object: self })
    }
}

impl<'a> UnitRef<'a> {
    #[inline]
    pub fn object(self) -> &'a Object {
        self.object
    }

    #[inline]
    pub fn pet_number(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::PetNumber))
            .filter(|value| *value != 0)
    }

    #[inline]
    pub fn health(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::Health))
    }

    #[inline]
    pub fn max_health(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::MaxHealth))
    }

    #[inline]
    pub fn power_type(self) -> Option<PowerType> {
        let bytes = field_bytes(self.object.unit_fields.get(&UnitField::Bytes0))?;
        PowerType::try_from(byte(bytes, 3)).ok()
    }

    #[inline]
    pub fn power(self) -> Option<u32> {
        self.power_by_type(self.power_type()?)
    }

    #[inline]
    pub fn max_power(self) -> Option<u32> {
        self.max_power_by_type(self.power_type()?)
    }

    #[inline]
    pub fn power_by_type(self, power_type: PowerType) -> Option<u32> {
        field_u32_array_at(
            self.object.unit_fields.get(&UnitField::Powers),
            u8::from(power_type) as usize,
        )
    }

    #[inline]
    pub fn max_power_by_type(self, power_type: PowerType) -> Option<u32> {
        field_u32_array_at(
            self.object.unit_fields.get(&UnitField::MaxPowers),
            u8::from(power_type) as usize,
        )
    }

    #[inline]
    pub fn level(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::Level))
    }

    #[inline]
    pub fn faction_template(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::FactionTemplate))
    }

    #[inline]
    pub fn npc_flags(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::NpcFlags))
    }

    #[inline]
    pub fn flags(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::Flags))
    }

    #[inline]
    pub fn flags2(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::Flags2))
    }

    #[inline]
    pub fn dynamic_flags(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::DynamicFlags))
    }

    #[inline]
    pub fn target_guid(self) -> Option<PackedGuid> {
        field_u64(self.object.unit_fields.get(&UnitField::Target)).map(PackedGuid)
    }

    /// Raw UNIT_FIELD_AURASTATE bitmask. This is not the unit's aura list.
    #[inline]
    pub fn aura_state(self) -> Option<u32> {
        field_u32(self.object.unit_fields.get(&UnitField::AuraState))
    }

    #[inline]
    pub fn position(self) -> Option<Point3D> {
        self.object.position()
    }

    #[inline]
    pub fn facing(self) -> Option<f32> {
        self.object.facing()
    }
}

impl<'a> PlayerRef<'a> {
    #[inline]
    pub fn object(self) -> &'a Object {
        self.object
    }

    #[inline]
    pub fn unit(self) -> UnitRef<'a> {
        UnitRef { object: self.object }
    }

    #[inline]
    pub fn race(self) -> Option<Race> {
        let bytes = field_bytes(self.object.unit_fields.get(&UnitField::Bytes0))?;
        Race::try_from(byte(bytes, 0)).ok()
    }

    #[inline]
    pub fn class(self) -> Option<Class> {
        let bytes = field_bytes(self.object.unit_fields.get(&UnitField::Bytes0))?;
        Class::try_from(byte(bytes, 1)).ok()
    }

    #[inline]
    pub fn gender(self) -> Option<Gender> {
        let bytes = field_bytes(self.object.unit_fields.get(&UnitField::Bytes0))?;
        Gender::try_from(byte(bytes, 2)).ok()
    }

    #[inline]
    pub fn level(self) -> Option<u32> {
        self.unit().level()
    }

    #[inline]
    pub fn health(self) -> Option<u32> {
        self.unit().health()
    }

    #[inline]
    pub fn max_health(self) -> Option<u32> {
        self.unit().max_health()
    }

    #[inline]
    pub fn power_type(self) -> Option<PowerType> {
        self.unit().power_type()
    }

    #[inline]
    pub fn power(self) -> Option<u32> {
        self.unit().power()
    }

    #[inline]
    pub fn max_power(self) -> Option<u32> {
        self.unit().max_power()
    }

    #[inline]
    pub fn target_guid(self) -> Option<PackedGuid> {
        self.unit().target_guid()
    }

    /// Visible item entry ID for a player equipment slot.
    #[inline]
    pub fn visible_item_entry(self, slot: usize) -> Option<u32> {
        let rows = match self.object.player_fields.get(&PlayerField::VisibleItems)? {
            FieldValue::CustomArray(rows) => rows,
            _ => return None,
        };
        let row = rows.get(slot)?;
        field_u32(row.first()?.as_ref())
    }

    /// Visible item enchantment pair for a player equipment slot.
    #[inline]
    pub fn visible_item_enchantment(self, slot: usize) -> Option<(i16, i16)> {
        let rows = match self.object.player_fields.get(&PlayerField::VisibleItems)? {
            FieldValue::CustomArray(rows) => rows,
            _ => return None,
        };
        let row = rows.get(slot)?;
        match row.get(1)?.as_ref()? {
            FieldValue::TwoShorts(value) => Some(*value),
            _ => None,
        }
    }

    #[inline]
    pub fn position(self) -> Option<Point3D> {
        self.object.position()
    }

    #[inline]
    pub fn facing(self) -> Option<f32> {
        self.object.facing()
    }
}

impl<'a> GameObjectRef<'a> {
    #[inline]
    pub fn object(self) -> &'a Object {
        self.object
    }

    #[inline]
    pub fn display_id(self) -> Option<u32> {
        field_u32(self.object.game_object_fields.get(&GameObjectField::DisplayId))
    }

    #[inline]
    pub fn game_object_type(self) -> Option<u8> {
        let bytes = field_bytes(self.object.game_object_fields.get(&GameObjectField::Bytes1))?;
        Some(byte(bytes, 1))
    }

    #[inline]
    pub fn state(self) -> Option<u8> {
        let bytes = field_bytes(self.object.game_object_fields.get(&GameObjectField::Bytes1))?;
        Some(byte(bytes, 0))
    }

    #[inline]
    pub fn flags(self) -> Option<u32> {
        field_u32(self.object.game_object_fields.get(&GameObjectField::Flags))
    }

    #[inline]
    pub fn faction(self) -> Option<u32> {
        field_u32(self.object.game_object_fields.get(&GameObjectField::Faction))
    }

    #[inline]
    pub fn position(self) -> Option<Point3D> {
        self.object.position()
    }

    #[inline]
    pub fn facing(self) -> Option<f32> {
        self.object.facing()
    }

    #[inline]
    pub fn rotation(self) -> Option<i64> {
        self.object.rotation()
    }

    #[inline]
    pub fn parent_rotation(self) -> Option<&'a [Option<f32>]> {
        match self
            .object
            .game_object_fields
            .get(&GameObjectField::ParentRotation)?
        {
            FieldValue::FloatArray(values) => Some(values.as_slice()),
            _ => None,
        }
    }
}

#[inline]
fn byte(value: u32, index: usize) -> u8 {
    ((value >> (index * 8)) & 0xFF) as u8
}

#[inline]
fn field_u32(value: Option<&FieldValue>) -> Option<u32> {
    match value? {
        FieldValue::Integer(value) => Some(*value as u32),
        _ => None,
    }
}

#[inline]
fn field_u64(value: Option<&FieldValue>) -> Option<u64> {
    match value? {
        FieldValue::Long(value) => Some(*value),
        _ => None,
    }
}

#[inline]
fn field_f32(value: Option<&FieldValue>) -> Option<f32> {
    match value? {
        FieldValue::Float(value) => Some(*value),
        _ => None,
    }
}

#[inline]
fn field_bytes(value: Option<&FieldValue>) -> Option<u32> {
    match value? {
        FieldValue::Bytes(value) => Some(*value),
        _ => None,
    }
}

#[inline]
fn field_u32_array_at(value: Option<&FieldValue>, index: usize) -> Option<u32> {
    match value? {
        FieldValue::IntegerArray(values) => values
            .get(index)?
            .as_ref()
            .map(|value| *value as u32),
        _ => None,
    }
}
