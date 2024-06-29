use crate::{Read, Write, Error, EnumString};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(strum::Display, Clone, Copy, Debug, EnumString)]
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
        println!("{}",string);
        Ok(std::str::FromStr::from_str(&string).unwrap())
    }
}
impl Write for Theme {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.to_string().write(output)
    }
}