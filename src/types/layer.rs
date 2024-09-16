use crate::{Read, Write, Error};
use crate::types::vec2::Vec2;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Layer {
    pub layer_id: i32,
    pub layer_name: String,
    pub selected: bool,
    pub invisible: bool,
    pub locked: bool,
    pub foreground_type: i32,
    pub parallax: Vec2,
    pub fixed_size: bool,
    pub children: Vec<i32>,
}

impl Read for Layer {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            layer_id: Read::read(input)?,
            layer_name: Read::read(input)?,
            selected: Read::read(input)?,
            invisible: Read::read(input)?,
            locked: Read::read(input)?,
            foreground_type: Read::read(input)?,
            parallax: Read::read(input)?,
            fixed_size: Read::read(input)?,
            children: Read::read(input)?,
        })
    }
}

impl Write for Layer {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.layer_id.write(output)?;
        self.layer_name.write(output)?;
        self.selected.write(output)?;
        self.invisible.write(output)?;
        self.locked.write(output)?;
        self.foreground_type.write(output)?;
        self.parallax.write(output)?;
        self.fixed_size.write(output)?;
        self.children.write(output)
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self { 
            layer_id: 1,
            layer_name: "".to_string(),
            selected: true,
            invisible: false, 
            locked: false,
            foreground_type: 0,
            parallax: Vec2 { x: 0.0.into(), y: 0.0.into() },
            fixed_size: false,
            children: vec![]
        }
    }
}