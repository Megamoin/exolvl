use crate::{Read, Write, Error};
use crate::types::call_parameter::CallParameter;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub id: i32,
    pub parameters: Vec<CallParameter>,
}

impl Read for FunctionCall {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            id: Read::read(input)?,
            parameters: Read::read(input)?,
        })
    }
}

impl Write for FunctionCall {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.id.write(output)?;
        self.parameters.write(output)
    }
}