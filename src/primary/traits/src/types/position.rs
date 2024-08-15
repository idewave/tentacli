use anyhow::{Result as AnyResult};
use std::io::{BufRead};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Serialize};

use crate::{BinaryConverter};

#[derive(Serialize, Copy, Clone, Default, Debug, PartialEq)]
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
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.x.write_into(buffer)?;
        self.y.write_into(buffer)?;
        self.z.write_into(buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let x = reader.read_f32::<LittleEndian>()?;
        let y = reader.read_f32::<LittleEndian>()?;
        let z = reader.read_f32::<LittleEndian>()?;

        Ok(Self { x, y, z })
    }
}

#[derive(Serialize,Copy, Clone, Default, Debug, PartialEq)]
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
    fn write_into(&mut self, buffer: &mut Vec<u8>) -> AnyResult<()> {
        self.point.write_into(buffer)?;
        self.direction.write_into(buffer)?;

        Ok(())
    }

    fn read_from<R: BufRead>(reader: &mut R, _: &mut Vec<u8>) -> AnyResult<Self> {
        let point = Point3D::read_from(reader, &mut vec![])?;
        let direction = reader.read_f32::<LittleEndian>()?;
        Ok(Self { point, direction })
    }
}