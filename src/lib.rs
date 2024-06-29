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

use std::time::Duration;
use error::Error;
#[cfg(feature = "image")]
use image::{DynamicImage, ImageFormat};
use strum::EnumString;
pub use traits::{Read, ReadContext, ReadVersioned, Write};
use uuid::Uuid;
use glam::{Vec2, IVec2};

use types::varint::Varint;

impl Read for String {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let len = Varint::read(input)?;

        let mut string = Self::with_capacity(usize::try_from(len.0).unwrap());

        for _ in 0..len.0 {
            let c = u8::read(input)? as char;
            string.push(c);
        }

        Ok(string)
    }
}

impl Write for String {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Varint(i32::try_from(self.len()).unwrap()).write(output)?;

        for c in self.chars() {
            (c as u8).write(output)?;
        }

        Ok(())
    }
}

impl Write for u32 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl Read for i32 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in &mut bytes {
            *byte = Read::read(input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for i32 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl Read for i64 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 8];

        for byte in &mut bytes {
            *byte = Read::read(input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for i64 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl Read for f32 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in &mut bytes {
            *byte = Read::read(input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for f32 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl<T: Read> Read for Vec<T> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let len = usize::try_from(i32::read(input)?).unwrap();

        let mut vec = Self::with_capacity(len);

        for _ in 0..len {
            vec.push(Read::read(input)?);
        }

        Ok(vec)
    }
}

impl<T: Write> Write for Vec<T> {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::try_from(self.len()).unwrap().write(output)?;

        for item in self {
            item.write(output)?;
        }

        Ok(())
    }
}

impl<T: Read + Copy + Default, const LEN: usize> Read for [T; LEN] {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut arr = [Default::default(); LEN];

        for item in &mut arr {
            *item = Read::read(input)?;
        }

        Ok(arr)
    }
}

impl<T: Write, const LEN: usize> Write for [T; LEN] {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        for item in self {
            item.write(output)?;
        }

        Ok(())
    }
}

impl<T: Read> Read for Option<T> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        if bool::read(input)? {
            Ok(Some(Read::read(input)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Write> Write for Option<T> {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.is_some().write(output)?;

        if let Some(value) = self {
            value.write(output)?;
        }

        Ok(())
    }
}

impl Read for bool {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(u8::read(input)? != 0)
    }
}

impl Write for bool {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        u8::from(*self).write(output)
    }
}

impl Read for u8 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut buf = [0; 1];
        input.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Write for u8 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&[*self])?)
    }
}

#[cfg(feature = "image")]
impl Read for DynamicImage {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let vec = Vec::<u8>::read(input)?;

        image::load_from_memory(&vec).map_err(Error::from)
    }
}

#[cfg(feature = "image")]
impl Write for DynamicImage {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let mut vec = std::io::Cursor::new(Vec::new());
        self.write_to(&mut vec, ImageFormat::Png)?;

        output.write_all(&vec.into_inner())?;

        Ok(())
    }
}


const TICKS_TO_SECONDS: i64 = 10_000_000;
const EPOCH_DIFFERENCE: i64 = 62_135_596_800;

impl Read for chrono::DateTime<chrono::Utc> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let ticks = i64::read(input)?;

        let seconds = ticks / TICKS_TO_SECONDS - EPOCH_DIFFERENCE;

        Ok(Self::from_timestamp(seconds, 0).unwrap())
    }
}

impl Write for chrono::DateTime<chrono::Utc> {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let ticks = (self.timestamp() + EPOCH_DIFFERENCE) * TICKS_TO_SECONDS;

        ticks.write(output)
    }
}


#[cfg(feature = "image")]
#[derive(Clone, Debug, PartialEq)]
pub struct Image(pub DynamicImage);

#[cfg(all(feature = "image", feature = "serde"))]
impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_bytes().serialize(serializer)
    }
}

#[cfg(all(feature = "image", feature = "serde"))]
impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buffer = serde::Deserialize::deserialize(deserializer)?;

        let img = image::load_from_memory(buffer).map_err(serde::de::Error::custom)?;

        Ok(Self(img))
    }
}

#[cfg(feature = "image")]
impl From<DynamicImage> for Image {
    fn from(value: DynamicImage) -> Self {
        Self(value)
    }
}

#[cfg(not(feature = "image"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image(pub Vec<u8>);

impl Read for Image {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let data = Read::read(input)?;

        Ok(Self(data))
    }
}

impl Write for Image {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.0.write(output)
    }
}


impl Read for Vec2 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Vec2::new(
            Read::read(input)?, 
            Read::read(input)?
        ))
    }
}

impl Write for Vec2 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}

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


impl Read for Uuid {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        Ok(uuid::Uuid::parse_str(&String::read(input)?).unwrap())
    }
}

impl Write for Uuid {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.to_string().write(output)
    }
}

impl Read for Duration {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        Ok(Duration::from_millis(i64::read(input)? as u64))
    }
}
impl Write for Duration {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        (self.as_millis() as i64).write(output)
    }
}