use ordered_float::OrderedFloat;

use crate::{Read, Write, Error};
use crate::types::{object_id::ObjectId, object_property::ObjectProperty, vec2::Vec2};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct Object {
    pub entity_id: i32,
    pub object_id: ObjectId,
    pub prefab_entity_id: i32,
    pub prefab_id: i32,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: OrderedFloat<f32>,
    pub tag: String,
    pub properties: Vec<ObjectProperty>,
    pub in_layer: i32,
    pub in_group: i32,
    pub group_members: Vec<i32>,
}

impl Read for Object {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            entity_id: Read::read(input)?,
            object_id: Read::read(input)?,
            prefab_entity_id: Read::read(input)?,
            prefab_id: Read::read(input)?,
            position: Read::read(input)?,
            scale: Read::read(input)?,
            rotation: Read::read(input)?,
            tag: Read::read(input)?,
            properties: Read::read(input)?,
            in_layer: Read::read(input)?,
            in_group: Read::read(input)?,
            group_members: Read::read(input)?,
        })
    }
}

impl Write for Object {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.entity_id.write(output)?;
        self.object_id.write(output)?;
        self.prefab_entity_id.write(output)?;
        self.prefab_id.write(output)?;
        self.position.write(output)?;
        self.scale.write(output)?;
        self.rotation.write(output)?;
        self.tag.write(output)?;
        self.properties.write(output)?;
        self.in_layer.write(output)?;
        self.in_group.write(output)?;
        self.group_members.write(output)
    }
}