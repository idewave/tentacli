use serde::Serialize;

use crate::types::movement::Movement;
use crate::types::update_data::UpdateData;

#[derive(Serialize, Clone, Default, Debug)]
pub struct Unit {
    pub guid: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Item {
    pub guid: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Container {
    pub guid: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct GameObject {
    pub guid: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct DynamicObject {
    pub guid: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
}

#[derive(Serialize, Clone, Default, Debug)]
pub struct Corpse {
    pub guid: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Movement::is_default")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "UpdateData::is_default")]
    pub update_data: UpdateData,
}