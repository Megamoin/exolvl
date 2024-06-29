use crate::{Read, Write, Error, Image};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "image", derive(Clone, Debug, PartialEq))]
#[cfg_attr(
    not(feature = "image"),
    derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)
)]
pub struct Pattern {
    pub pattern_id: i32,
    pub pattern_frames: Vec<Image>,
}

impl Read for Pattern {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            pattern_id: Read::read(input)?,
            pattern_frames: Read::read(input)?,
        })
    }
}

impl Write for Pattern {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.pattern_id.write(output)?;
        self.pattern_frames.write(output)
    }
}

impl From<Vec<Image>> for Pattern {
    fn from(value: Vec<Image>) -> Self {
        Self { 
            pattern_id: rand::random::<i32>(), 
            pattern_frames: value,
        }
    }
}