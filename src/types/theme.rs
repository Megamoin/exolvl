use crate::traits::{Read, Write};
use crate::error::Error;
use strum::{Display, EnumString};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Display, EnumString, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "lowercase")]
pub enum Theme {
    Mountains,
    Halloween,
    Christmas,
    Custom,
}
impl Read for Theme {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
    Self: Sized {
        let string = String::read(input)?;
        string.parse::<Theme>().map_err( |e| Error::StrumParse(e))
    }
}
impl Write for Theme {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.to_string().write(output)
    }
}
impl Default for Theme {
    fn default() -> Self {
        Self::Mountains
    }
}