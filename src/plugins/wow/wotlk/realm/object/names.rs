use std::collections::{HashMap, HashSet};

use crate::plugins::wow::wotlk::realm::object::name_query::NameQueryRequest;
use crate::plugins::wow::wotlk::realm::object::types::update_data::UpdateData;
use crate::plugins::wow::wotlk::realm::object::types::update_fields::{
    FieldValue, ObjectField, UnitField,
};
use crate::plugins::wow::wotlk::realm::object::{Object, ObjectTypeId, PackedGuid};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerIdentity {
    name: String,
    realm_name: String,
}

impl PlayerIdentity {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn realm_name(&self) -> &str {
        &self.realm_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetIdentity {
    name: String,
    name_timestamp: u32,
}

impl PetIdentity {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn name_timestamp(&self) -> u32 {
        self.name_timestamp
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureIdentity {
    name: String,
}

impl CreatureIdentity {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameObjectIdentity {
    name: String,
    game_object_type: u32,
    display_id: u32,
}

impl GameObjectIdentity {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn game_object_type(&self) -> u32 {
        self.game_object_type
    }

    #[inline]
    pub fn display_id(&self) -> u32 {
        self.display_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemIdentity {
    name: String,
    inventory_type: u32,
}

impl ItemIdentity {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn inventory_type(&self) -> u32 {
        self.inventory_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NameQueryKey {
    Player(PackedGuid),
    Pet(u32),
    Creature(u32),
    GameObject(u32),
    Item(u32),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NameQuerySnapshot {
    object_type_id: ObjectTypeId,
    guid: PackedGuid,
    entry: Option<u32>,
    pet_number: Option<u32>,
    pet_name_timestamp: Option<u32>,
    schedule_primary: bool,
    schedule_pet: bool,
    refresh_pet_name: bool,
}

impl NameQuerySnapshot {
    pub(crate) fn from_object(object: &Object) -> Self {
        Self {
            object_type_id: object.object_type_id,
            guid: object.guid,
            entry: entry_id(object),
            pet_number: pet_number(object),
            pet_name_timestamp: pet_name_timestamp(object),
            schedule_primary: true,
            schedule_pet: true,
            refresh_pet_name: false,
        }
    }

    pub(crate) fn updated(mut self, update: &UpdateData) -> Option<Self> {
        let entry_updated = update.object_fields.contains_key(&ObjectField::Entry);
        let pet_number_updated = update.unit_fields.contains_key(&UnitField::PetNumber);
        let pet_name_timestamp_updated = update
            .unit_fields
            .contains_key(&UnitField::PetNameTimestamp);

        if !entry_updated && !pet_number_updated && !pet_name_timestamp_updated {
            return None;
        }

        self.schedule_primary = entry_updated;
        self.schedule_pet = pet_number_updated || pet_name_timestamp_updated;
        self.refresh_pet_name = pet_name_timestamp_updated;

        if let Some(FieldValue::Integer(value)) = update.object_fields.get(&ObjectField::Entry) {
            self.entry = (*value > 0).then_some(*value as u32);
        }

        if let Some(FieldValue::Integer(value)) = update.unit_fields.get(&UnitField::PetNumber) {
            self.pet_number = (*value > 0).then_some(*value as u32);
        }

        if let Some(FieldValue::Integer(value)) =
            update.unit_fields.get(&UnitField::PetNameTimestamp)
        {
            self.pet_name_timestamp = Some(*value as u32);
        }

        Some(self)
    }
}

#[derive(Default)]
pub struct ObjectNameRegistry {
    players: HashMap<PackedGuid, PlayerIdentity>,
    pets: HashMap<u32, PetIdentity>,
    creatures: HashMap<u32, CreatureIdentity>,
    game_objects: HashMap<u32, GameObjectIdentity>,
    items: HashMap<u32, ItemIdentity>,
    pending: HashSet<NameQueryKey>,
    failed: HashSet<NameQueryKey>,
}

impl ObjectNameRegistry {
    #[inline]
    pub fn player(&self, guid: PackedGuid) -> Option<&PlayerIdentity> {
        self.players.get(&guid)
    }

    #[inline]
    pub fn pet(&self, pet_number: u32) -> Option<&PetIdentity> {
        self.pets.get(&pet_number)
    }

    #[inline]
    pub fn creature(&self, entry: u32) -> Option<&CreatureIdentity> {
        self.creatures.get(&entry)
    }

    #[inline]
    pub fn game_object(&self, entry: u32) -> Option<&GameObjectIdentity> {
        self.game_objects.get(&entry)
    }

    #[inline]
    pub fn item(&self, entry: u32) -> Option<&ItemIdentity> {
        self.items.get(&entry)
    }

    /// Resolves the best cached name for an object without allocating or cloning.
    pub fn name_for<'a>(&'a self, object: &Object) -> Option<&'a str> {
        match object.object_type_id {
            ObjectTypeId::Player => self.player(object.guid).map(PlayerIdentity::name),
            ObjectTypeId::Unit => {
                if let Some(pet_number) = pet_number(object)
                    && let Some(identity) = self.pet(pet_number)
                {
                    return Some(identity.name());
                }

                self.creature(entry_id(object)?).map(CreatureIdentity::name)
            }
            ObjectTypeId::GameObject => self
                .game_object(entry_id(object)?)
                .map(GameObjectIdentity::name),
            ObjectTypeId::Item | ObjectTypeId::Container => {
                self.item(entry_id(object)?).map(ItemIdentity::name)
            }
            _ => None,
        }
    }

    pub(crate) fn schedule_for(&mut self, object: &Object) -> Vec<NameQueryRequest> {
        self.schedule_state(NameQuerySnapshot::from_object(object))
    }

    pub(crate) fn schedule_state(&mut self, state: NameQuerySnapshot) -> Vec<NameQueryRequest> {
        let mut requests = Vec::with_capacity(2);

        match state.object_type_id {
            ObjectTypeId::Player => {
                if state.schedule_primary {
                    let key = NameQueryKey::Player(state.guid);
                    if self.should_schedule(key) {
                        requests.push(NameQueryRequest::Player { guid: state.guid });
                    }
                }
            }
            ObjectTypeId::Unit => {
                if state.schedule_primary
                    && let Some(entry) = state.entry
                {
                    let key = NameQueryKey::Creature(entry);
                    if self.should_schedule(key) {
                        requests.push(NameQueryRequest::Creature {
                            entry,
                            guid: state.guid,
                        });
                    }
                }

                if state.schedule_pet
                    && let Some(pet_number) = state.pet_number
                {
                    let key = NameQueryKey::Pet(pet_number);

                    if state.refresh_pet_name {
                        if let (Some(identity), Some(name_timestamp)) =
                            (self.pets.get(&pet_number), state.pet_name_timestamp)
                            && identity.name_timestamp != name_timestamp
                        {
                            self.pets.remove(&pet_number);
                        }

                        self.pending.remove(&key);
                        self.failed.remove(&key);
                    }

                    if self.should_schedule(key) {
                        requests.push(NameQueryRequest::Pet {
                            pet_number,
                            guid: state.guid,
                        });
                    }
                }
            }
            ObjectTypeId::GameObject => {
                if state.schedule_primary
                    && let Some(entry) = state.entry
                {
                    let key = NameQueryKey::GameObject(entry);
                    if self.should_schedule(key) {
                        requests.push(NameQueryRequest::GameObject {
                            entry,
                            guid: state.guid,
                        });
                    }
                }
            }
            ObjectTypeId::Item | ObjectTypeId::Container => {
                if state.schedule_primary
                    && let Some(entry) = state.entry
                {
                    let key = NameQueryKey::Item(entry);
                    if self.should_schedule(key) {
                        requests.push(NameQueryRequest::Item {
                            entry,
                            guid: state.guid,
                        });
                    }
                }
            }
            _ => {}
        }

        requests
    }

    fn should_schedule(&mut self, key: NameQueryKey) -> bool {
        if self.pending.contains(&key) || self.failed.contains(&key) || self.contains(key) {
            return false;
        }

        self.pending.insert(key);
        true
    }

    fn contains(&self, key: NameQueryKey) -> bool {
        match key {
            NameQueryKey::Player(guid) => self.players.contains_key(&guid),
            NameQueryKey::Pet(pet_number) => self.pets.contains_key(&pet_number),
            NameQueryKey::Creature(entry) => self.creatures.contains_key(&entry),
            NameQueryKey::GameObject(entry) => self.game_objects.contains_key(&entry),
            NameQueryKey::Item(entry) => self.items.contains_key(&entry),
        }
    }

    pub(crate) fn resolve_player(
        &mut self,
        guid: PackedGuid,
        name: String,
        realm_name: String,
    ) {
        let key = NameQueryKey::Player(guid);
        self.pending.remove(&key);
        self.failed.remove(&key);
        self.players.insert(
            guid,
            PlayerIdentity { name, realm_name },
        );
    }

    pub(crate) fn resolve_pet(&mut self, pet_number: u32, name: String, name_timestamp: u32) {
        let key = NameQueryKey::Pet(pet_number);
        self.pending.remove(&key);
        self.failed.remove(&key);
        self.pets.insert(
            pet_number,
            PetIdentity {
                name,
                name_timestamp,
            },
        );
    }

    pub(crate) fn resolve_creature(&mut self, entry: u32, name: String) {
        let key = NameQueryKey::Creature(entry);
        self.pending.remove(&key);
        self.failed.remove(&key);
        self.creatures.insert(entry, CreatureIdentity { name });
    }

    pub(crate) fn resolve_game_object(
        &mut self,
        entry: u32,
        name: String,
        game_object_type: u32,
        display_id: u32,
    ) {
        let key = NameQueryKey::GameObject(entry);
        self.pending.remove(&key);
        self.failed.remove(&key);
        self.game_objects.insert(
            entry,
            GameObjectIdentity {
                name,
                game_object_type,
                display_id,
            },
        );
    }

    pub(crate) fn resolve_item(&mut self, entry: u32, name: String, inventory_type: u32) {
        let key = NameQueryKey::Item(entry);
        self.pending.remove(&key);
        self.failed.remove(&key);
        self.items.insert(
            entry,
            ItemIdentity {
                name,
                inventory_type,
            },
        );
    }

    pub(crate) fn fail_player(&mut self, guid: PackedGuid) {
        self.fail(NameQueryKey::Player(guid));
    }

    pub(crate) fn fail_pet(&mut self, pet_number: u32) {
        self.fail(NameQueryKey::Pet(pet_number));
    }

    pub(crate) fn fail_creature(&mut self, entry: u32) {
        self.fail(NameQueryKey::Creature(entry));
    }

    pub(crate) fn fail_game_object(&mut self, entry: u32) {
        self.fail(NameQueryKey::GameObject(entry));
    }

    fn fail(&mut self, key: NameQueryKey) {
        self.pending.remove(&key);
        self.failed.insert(key);
    }
}

fn entry_id(object: &Object) -> Option<u32> {
    object.entry_id().filter(|entry| *entry != 0)
}

fn pet_number(object: &Object) -> Option<u32> {
    match object.unit_fields.get(&UnitField::PetNumber)? {
        FieldValue::Integer(value) if *value > 0 => Some(*value as u32),
        _ => None,
    }
}

fn pet_name_timestamp(object: &Object) -> Option<u32> {
    match object.unit_fields.get(&UnitField::PetNameTimestamp)? {
        FieldValue::Integer(value) => Some(*value as u32),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::plugins::wow::wotlk::realm::object::types::update_data::ObjectTypeMask;
    use crate::plugins::wow::wotlk::realm::object::types::update_fields::ObjectField;

    fn object(guid: u64, object_type_id: ObjectTypeId, entry: Option<u32>) -> Object {
        let mut object = Object {
            guid: PackedGuid(guid),
            object_type_id,
            object_type_mask: ObjectTypeMask::OBJECT,
            movement: None,
            object_fields: BTreeMap::new(),
            unit_fields: BTreeMap::new(),
            player_fields: BTreeMap::new(),
            item_fields: BTreeMap::new(),
            container_fields: BTreeMap::new(),
            game_object_fields: BTreeMap::new(),
            dynamic_object_fields: BTreeMap::new(),
            corpse_fields: BTreeMap::new(),
        };

        if let Some(entry) = entry {
            object
                .object_fields
                .insert(ObjectField::Entry, FieldValue::Integer(entry as i32));
        }

        object
    }

    #[test]
    fn duplicate_creature_entries_schedule_one_query() {
        let mut registry = ObjectNameRegistry::default();
        let first = object(1, ObjectTypeId::Unit, Some(123));
        let second = object(2, ObjectTypeId::Unit, Some(123));

        assert_eq!(registry.schedule_for(&first).len(), 1);
        assert!(registry.schedule_for(&second).is_empty());
    }

    #[test]
    fn cached_creature_name_is_shared_by_all_matching_objects() {
        let mut registry = ObjectNameRegistry::default();
        let first = object(1, ObjectTypeId::Unit, Some(123));
        let second = object(2, ObjectTypeId::Unit, Some(123));

        registry.resolve_creature(123, "Guard".to_string());

        assert_eq!(registry.name_for(&first), Some("Guard"));
        assert_eq!(registry.name_for(&second), Some("Guard"));
        assert!(registry.schedule_for(&first).is_empty());
    }

    #[test]
    fn player_names_are_keyed_by_guid() {
        let mut registry = ObjectNameRegistry::default();
        let first = object(10, ObjectTypeId::Player, None);
        let second = object(11, ObjectTypeId::Player, None);

        assert_eq!(registry.schedule_for(&first).len(), 1);
        assert_eq!(registry.schedule_for(&second).len(), 1);

        registry.resolve_player(PackedGuid(10), "Alice".to_string(), String::new());

        assert_eq!(registry.name_for(&first), Some("Alice"));
        assert_eq!(registry.name_for(&second), None);
    }

    #[test]
    fn pet_name_overrides_creature_template_name() {
        let mut registry = ObjectNameRegistry::default();
        let mut pet = object(20, ObjectTypeId::Unit, Some(456));
        pet.unit_fields
            .insert(UnitField::PetNumber, FieldValue::Integer(777));

        let requests = registry.schedule_for(&pet);
        assert_eq!(requests.len(), 2);

        registry.resolve_creature(456, "Wolf".to_string());
        assert_eq!(registry.name_for(&pet), Some("Wolf"));

        registry.resolve_pet(777, "Bobby".to_string(), 42);
        assert_eq!(registry.name_for(&pet), Some("Bobby"));
    }

    #[test]
    fn pet_number_from_values_schedules_pet_query() {
        let mut registry = ObjectNameRegistry::default();
        let pet = object(21, ObjectTypeId::Unit, Some(456));

        assert_eq!(registry.schedule_for(&pet).len(), 1);

        let mut update = UpdateData::default();
        update
            .unit_fields
            .insert(UnitField::PetNumber, FieldValue::Integer(778));

        let state = NameQuerySnapshot::from_object(&pet).updated(&update).unwrap();
        let requests = registry.schedule_state(state);

        assert_eq!(
            requests,
            vec![NameQueryRequest::Pet {
                pet_number: 778,
                guid: PackedGuid(21),
            }]
        );
    }

    #[test]
    fn pet_name_timestamp_change_refreshes_cached_name() {
        let mut registry = ObjectNameRegistry::default();
        let mut pet = object(22, ObjectTypeId::Unit, Some(456));
        pet.unit_fields
            .insert(UnitField::PetNumber, FieldValue::Integer(779));
        pet.unit_fields
            .insert(UnitField::PetNameTimestamp, FieldValue::Integer(42));

        registry.resolve_pet(779, "Old name".to_string(), 42);

        let mut update = UpdateData::default();
        update
            .unit_fields
            .insert(UnitField::PetNameTimestamp, FieldValue::Integer(43));

        let state = NameQuerySnapshot::from_object(&pet).updated(&update).unwrap();
        let requests = registry.schedule_state(state);

        assert_eq!(
            requests,
            vec![NameQueryRequest::Pet {
                pet_number: 779,
                guid: PackedGuid(22),
            }]
        );
        assert!(registry.pet(779).is_none());
    }

    #[test]
    fn failed_lookup_is_not_rescheduled() {
        let mut registry = ObjectNameRegistry::default();
        let creature = object(30, ObjectTypeId::Unit, Some(999));

        assert_eq!(registry.schedule_for(&creature).len(), 1);
        registry.fail_creature(999);
        assert!(registry.schedule_for(&creature).is_empty());
    }
}
