use std::{io::Write as _, path::Path};

use crate::{error::Error, Read, Write};
use image::ImageReader;
#[cfg(feature = "image")]
use image::{DynamicImage, RgbaImage};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{de::Visitor, Serialize};

#[cfg(feature = "image")]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Image(pub RgbaImage);


#[cfg(all(feature = "image", feature = "serde"))]
impl Serialize for Image {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    
        let mut wrapped_writer = vec![];
        {
            let mut image_data: Vec<u8> = Vec::new();
            self.0.write_to(&mut std::io::Cursor::new(&mut image_data), image::ImageFormat::Png).unwrap();
            let mut enc = base64::write::EncoderWriter::new(&mut wrapped_writer, &BASE64_STANDARD);

            enc.write_all(&image_data).unwrap();

            enc.finish().unwrap();

        }
        String::from_utf8(wrapped_writer).unwrap().serialize(serializer)
    }
}

/*
#[cfg(all(feature = "image", feature = "serde"))]
impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_vec().serialize(serializer)
    }
}
*/

impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ImageVisitor;

        impl<'de> Visitor<'de> for ImageVisitor {
            type Value = Image;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a Base64 encoded image string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let decoded_data = base64::engine::general_purpose::STANDARD.decode(v).map_err(serde::de::Error::custom)?;
                
                let image = image::load_from_memory(&decoded_data).map_err(serde::de::Error::custom)?;
                
                let rgba_image = image.to_rgba8();

                Ok(Image(rgba_image))
            }
        }

        deserializer.deserialize_str(ImageVisitor)
    }
}

/*
#[cfg(all(feature = "image", feature = "serde"))]
impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buffer = serde::Deserialize::deserialize(deserializer)?;

        let img = image::load_from_memory(buffer).map_err(serde::de::Error::custom)?;

        let img = img.to_rgba8();

        Ok(Self(img))
    }
}
*/

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
        let mut buffer = Vec::new();

        self.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)?;

        buffer.write(output)?;

        Ok(())
    }
}

#[cfg(feature = "image")]
impl Read for RgbaImage {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let vec = Vec::<u8>::read(input)?;

        image::load_from_memory(&vec)
            .map_err(Error::from)
            .map(|img| img.to_rgba8())
    }
}

#[cfg(feature = "image")]
impl Write for RgbaImage {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let mut buffer = Vec::new();

        self.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)?;

        buffer.write(output)?;

        Ok(())
    }
}

#[cfg(feature = "image")]
impl From<DynamicImage> for Image {
    fn from(value: DynamicImage) -> Self {
        Self(value.into())
    }
}

#[cfg(feature = "image")]
impl Image {
    pub fn to_base64_string(&self) -> String {
        let mut wrapped_writer = vec![];
        {
            let mut image_data: Vec<u8> = Vec::new();
            self.0.write_to(&mut std::io::Cursor::new(&mut image_data), image::ImageFormat::Png).unwrap();
            let mut enc = base64::write::EncoderWriter::new(&mut wrapped_writer, &BASE64_STANDARD);
            
            enc.write_all(&image_data).unwrap();
            
            enc.finish().unwrap();

        }
        String::from_utf8(wrapped_writer).unwrap()
    }

    pub fn read_from_file(path: &Path) -> Self {
        ImageReader::open(path).unwrap().decode().unwrap().into()
    }
}