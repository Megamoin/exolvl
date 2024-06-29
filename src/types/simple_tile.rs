use crate::{Read, Write, Error, IVec2};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct SimpleTile {
    pub position: IVec2,
    pub tile_id: String,
}

impl Read for SimpleTile {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        Ok(Self {
            position: Read::read(input)?,
            tile_id: Read::read(input)?,
        })
    }
}

impl Write for SimpleTile {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.position.write(output)?;
        self.tile_id.write(output)
    }
}