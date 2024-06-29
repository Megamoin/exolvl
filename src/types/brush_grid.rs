use crate::{Read, Write, Error};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BrushGrid {
    pub x: i32,
    pub y: i32,
}

impl Read for BrushGrid {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            x: Read::read(input)?,
            y: Read::read(input)?,
        })
    }
}

impl Write for BrushGrid {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}