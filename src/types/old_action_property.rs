use crate::{Read, Write, Error};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OldActionProperty {
    pub name: String,
    pub value: String,
}

impl Read for OldActionProperty {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        Ok(Self { 
            name: Read::read(input)?,
            value: Read::read(input)?,
        })
    }
}

impl Write for OldActionProperty {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.name.write(output)?;
        self.value.write(output)
    }
}