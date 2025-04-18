use std::default;

use crate::{Error, Read, Write};
use crate::types::vec2::Vec2;
use crate::types::color::Color;
use super::dynamic_type::DynamicType;
use ordered_float::OrderedFloat;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct NovaValue {
    pub dynamic_type: DynamicType,

    pub bool_value: bool,
    pub int_value: i32,
    pub float_value: OrderedFloat<f32>,
    pub string_value: Option<String>,
    pub color_value: Color,
    pub vector_value: Vec2,
    pub bool_list_values: Option<Vec<bool>>, // new
    pub int_list_values: Option<Vec<i32>>,
    pub float_list_values: Option<Vec<OrderedFloat<f32>>>, // new
    pub string_list_values: Option<Vec<String>>, // new
    pub color_list_values: Option<Vec<Color>>, // new
    pub vector_list_values: Option<Vec<Vec2>>, // new
    pub sub_values: Option<Vec<NovaValue>>,
}

///! Turn this into a ReadVersioned trait
impl Read for NovaValue {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            dynamic_type: Read::read(input)?,
            bool_value: Read::read(input)?,
            int_value: Read::read(input)?,
            float_value: Read::read(input)?,
            string_value: Read::read(input)?,
            color_value: Read::read(input)?,
            vector_value: Read::read(input)?,
            bool_list_values: Read::read(input)?,
            int_list_values: Read::read(input)?,
            float_list_values: Read::read(input)?,
            string_list_values: Read::read(input)?,
            color_list_values: Read::read(input)?,
            vector_list_values: Read::read(input)?,
            sub_values: Read::read(input)?,
        })
    }
}

impl Write for NovaValue {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.dynamic_type.write(output)?;
        self.bool_value.write(output)?;
        self.int_value.write(output)?;
        self.float_value.write(output)?;
        self.string_value.write(output)?;
        self.color_value.write(output)?;
        self.vector_value.write(output)?;
        self.bool_list_values.write(output)?;
        self.int_list_values.write(output)?;
        self.float_list_values.write(output)?;
        self.string_list_values.write(output)?;
        self.color_list_values.write(output)?;
        self.vector_list_values.write(output)?;
        self.sub_values.write(output)
    }
}

