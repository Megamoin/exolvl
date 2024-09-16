use crate::{Read, Write, Error, IVec2};
use crate::types::vec2::Vec2;
use super::object_tile_property::ObjectTileProperty;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ObjectTile {
    pub position: IVec2,
    pub tile_id: String,
    pub entity_id: i32,
    pub offset: Vec2,
    pub properties: Vec<ObjectTileProperty>,
}

impl Read for ObjectTile {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        Ok(Self {
            position: Read::read(input)?,
            tile_id: Read::read(input)?,
            entity_id: Read::read(input)?,
            offset: Read::read(input)?,
            properties: Read::read(input)?,
        })
    }
}

impl Write for ObjectTile {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.position.write(output)?;
        self.tile_id.write(output)?;
        self.entity_id.write(output)?;
        self.offset.write(output)?;
        self.properties.write(output)
    }
}