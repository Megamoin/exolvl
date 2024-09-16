#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// #![warn(missing_docs)] // uncomment when writing docs
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]
#![cfg_attr(target_os = "windows", doc=include_str!("..\\README.md"))]
#![cfg_attr(not(target_os = "windows"), doc=include_str!("../README.md"))]

pub mod error;
mod private;
pub mod traits;
pub mod encrypt;
pub mod types;
pub mod geometrize;
pub mod primitive_impls;

use error::Error;
#[cfg(feature = "image")]
use strum::EnumString;
pub use traits::{Read, ReadContext, ReadVersioned, Write};
use uuid::Uuid;
use glam::{Vec2, IVec2};


impl Read for IVec2 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(IVec2::new(
            Read::read(input)?, 
            Read::read(input)?
        ))
    }
}

impl Write for IVec2 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}
