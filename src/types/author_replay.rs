use crate::{Read, Write, Error};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorReplay(pub Vec<u8>);

impl Read for AuthorReplay {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self(Read::read(input)?))
    }
}

impl Write for AuthorReplay {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.0.write(output)
    }
}