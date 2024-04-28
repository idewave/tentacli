use anyhow::{Result as AnyResult};
use std::io::{BufRead};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::{BinaryConverter};

#[derive(Copy, Clone, Default, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl BinaryConverter for Point3D {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let x = reader.read_f32::<LittleEndian>()?;
        let y = reader.read_f32::<LittleEndian>()?;
        let z = reader.read_f32::<LittleEndian>()?;

        Ok(Self { x, y, z })
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for Point3D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 3;
        let mut state = serializer.serialize_struct("Point3D", FIELDS_AMOUNT)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.end()
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Vector3D {
    pub point: Point3D,
    pub direction: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32, direction: f32) -> Self {
        Self { point: Point3D::new(x, y, z), direction }
    }
}

impl BinaryConverter for Vector3D {
    fn write_into(&mut self, _: &mut Vec<u8>) -> AnyResult<()> {
        todo!()
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let point = Point3D::read_from(reader, &mut vec![])?;
        let direction = reader.read_f32::<LittleEndian>()?;
        Ok(Self { point, direction })
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Serialize for Vector3D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        const FIELDS_AMOUNT: usize = 2;
        let mut state = serializer.serialize_struct("Vector3D", FIELDS_AMOUNT)?;
        state.serialize_field("point", &self.point)?;
        state.serialize_field("direction", &self.direction)?;
        state.end()
    }
}