use crate::{Read, Write, Error};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariablesFolder {
    pub folder_id: i32,
    pub folder_name: String,
    pub closed: bool,   
    pub variables: Vec<i32>,
}

impl Read for VariablesFolder {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            folder_id: Read::read(input)?,
            folder_name: Read::read(input)?,
            closed: Read::read(input)?,
            variables: Read::read(input)?,
        })
    }
}

impl Write for VariablesFolder {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.folder_id.write(output)?;
        self.folder_name.write(output)?;
        self.closed.write(output)?;
        self.variables.write(output)
    }
}