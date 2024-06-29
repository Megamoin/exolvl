use crate::{Read, Write, Error};
use crate::types::{old_actiontype::OldActionType, old_action_property::OldActionProperty};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OldAction {
    pub action_type: OldActionType,
    pub wait: bool,
    pub properties: Vec<OldActionProperty>,
}

impl Read for OldAction {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        Ok(Self { 
            action_type: Read::read(input)?,
            wait: Read::read(input)?,
            properties: Read::read(input)?,
        })
    }
}

impl Write for OldAction {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.action_type.write(output)?;
        self.wait.write(output)?;
        self.properties.write(output)
    }
}