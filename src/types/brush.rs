use crate::{Read, Write, Error, Vec2};
use crate::types::{brush_grid::BrushGrid, brush_object::BrushObject};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Brush {
    pub brush_id: i32,
    pub spread: Vec2,
    pub frequency: f32,
    pub grid: BrushGrid,
    pub objects: Vec<BrushObject>,
}

impl Read for Brush {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            brush_id: Read::read(input)?,
            spread: Read::read(input)?,
            frequency: Read::read(input)?,
            grid: Read::read(input)?,
            objects: Read::read(input)?,
        })
    }
}

impl Write for Brush {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.brush_id.write(output)?;
        self.spread.write(output)?;
        self.frequency.write(output)?;
        self.grid.write(output)?;
        self.objects.write(output)
    }
}