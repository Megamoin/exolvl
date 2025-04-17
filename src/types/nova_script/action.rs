use crate::{Error, Read, ReadContext, ReadVersioned, Write};
use super::action_type::ActionType;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Action {
    pub closed: bool,
    pub wait: bool,
    pub action_type: ActionType,
}

impl ReadVersioned for Action {
    fn read(input: &mut impl std::io::Read, version: i32) -> Result<Self, Error> {
        let action_type = Read::read(input)?;

        Ok(Self {
            closed: Read::read(input)?,
            wait: Read::read(input)?,
            action_type: ReadContext::read_ctx(input, action_type, version)?,
        })
    }
}

impl Write for Action {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let action_type = i32::from(&self.action_type);

        action_type.write(output)?;
        self.closed.write(output)?;
        self.wait.write(output)?;
        self.action_type.write(output)
    }
}