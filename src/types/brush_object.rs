use crate::{Read, Write, Error};
use crate::types::object_property::ObjectProperty;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct BrushObject {
    pub entity_id: i32,
    pub properties: Vec<ObjectProperty>,
    pub weight: f32,
    pub scale: f32,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Read for BrushObject {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            entity_id: Read::read(input)?,
            properties: Read::read(input)?,
            weight: Read::read(input)?,
            scale: Read::read(input)?,
            rotation: Read::read(input)?,
            flip_x: Read::read(input)?,
            flip_y: Read::read(input)?,
        })
    }
}

impl Write for BrushObject {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.entity_id.write(output)?;
        self.properties.write(output)?;
        self.weight.write(output)?;
        self.scale.write(output)?;
        self.rotation.write(output)?;
        self.flip_x.write(output)?;
        self.flip_y.write(output)
    }
}
