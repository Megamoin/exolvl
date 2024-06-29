use crate::{Read, Write, Error};
use crate::types::old_action::OldAction;


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Script {
    pub script_id: uuid::Uuid,
    pub name: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub actions: Vec<OldAction>,
}

impl Read for Script {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        Ok(Self { 
            script_id: Read::read(input)?,
            name: Read::read(input)?,
            creation_date: Read::read(input)?,
            actions: Read::read(input)?,
        })
    }
}

impl Write for Script {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.script_id.write(output)?;
        self.name.write(output)?;
        self.creation_date.write(output)?;
        self.actions.write(output)
    }
}