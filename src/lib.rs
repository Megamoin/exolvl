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
pub mod types;
#[cfg(feature = "image")]
pub mod geometrize;
pub mod primitive_impls;

// ! These modules are only included when the "private-modules" feature is enabled
#[cfg(feature = "private-modules")]
pub mod encrypt;
#[cfg(feature = "private-modules")]
pub mod request;
#[cfg(feature = "private-modules")]
pub mod gui;


use error::Error;
pub use traits::{Read, ReadContext, ReadVersioned, Write};
use uuid::Uuid;
use glam::IVec2;

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
