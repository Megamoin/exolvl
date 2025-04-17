pub mod action_type;
pub mod action;
pub mod activator;
pub mod parameter;
pub mod nova_value;
pub mod variable;
pub mod function_call;
pub mod static_type;
pub mod dynamic_type;
pub mod scripts_folder;
pub mod variables_folder;

use crate::{Error, Read, ReadVersioned, Write};
use {nova_value::NovaValue, activator::Activator, parameter::Parameter, variable::Variable, action::Action};


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NovaScript {
    pub script_id: i32,
    pub script_name: String,
    pub is_function: bool,
    pub activation_count: i32,
    pub condition: NovaValue,
    pub activation_list: Vec<Activator>,
    pub parameters: Vec<Parameter>,
    // pub variables_folder: Option<Vec<variables_folder::VariablesFolder>>,
    pub variables: Vec<Variable>,
    pub actions: Vec<Action>,
}

impl ReadVersioned for NovaScript {
    fn read(input: &mut impl std::io::Read, version: i32) -> Result<Self, Error> {
        Ok(Self {
            script_id: Read::read(input)?,
            script_name: Read::read(input)?,
            is_function: Read::read(input)?,
            activation_count: Read::read(input)?,
            condition: Read::read(input)?,
            activation_list: Read::read(input)?,
            parameters: Read::read(input)?,
            // variables_folder: if version >= 19 {
            //     None // Some(Read::read(input)?)
            // } else {
            //     None
            // },
            variables: ReadVersioned::read(input, version)?,
            actions: ReadVersioned::read(input, version)?,
        })
    }
}

impl Write for NovaScript {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.script_id.write(output)?;
        self.script_name.write(output)?;
        self.is_function.write(output)?;
        self.activation_count.write(output)?;
        self.condition.write(output)?;
        self.activation_list.write(output)?;
        self.parameters.write(output)?;
        // if let Some(variables_folder) = &self.variables_folder {
        //     variables_folder.write(output)?;
        // }
        self.variables.write(output)?;
        self.actions.write(output)
    }
}