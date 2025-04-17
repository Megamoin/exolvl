use crate::{Error, Read, ReadVersioned, Write};
use super::{static_type::StaticType, nova_value::NovaValue};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable {
    pub variable_id: i32,
    pub name: String,
    pub static_type: StaticType,
    pub persistance: Option<i32>,
    pub initial_value: NovaValue,
}

impl ReadVersioned for Variable {
    fn read(input: &mut impl std::io::Read, version: i32) -> Result<Self, Error> {
        Ok(Self {
            variable_id: Read::read(input)?,
            name: Read::read(input)?,
            static_type: Read::read(input)?,
            persistance: if version >= 19 {
                Some(Read::read(input)?)
            } else {
                None
            },
            initial_value: Read::read(input)?,
        })
    }
}

impl Write for Variable {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.variable_id.write(output)?;
        self.name.write(output)?;
        self.static_type.write(output)?;
        if let Some(persistance) = self.persistance {
            persistance.write(output)?;
        }
        self.initial_value.write(output)
    }
}