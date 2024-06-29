use crate::{Read, Write, Error};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Read for Color {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            r: Read::read(input)?,
            g: Read::read(input)?,
            b: Read::read(input)?,
            a: Read::read(input)?,
        })
    }
}

impl Write for Color {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.r.write(output)?;
        self.g.write(output)?;
        self.b.write(output)?;
        self.a.write(output)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl std::str::FromStr for Color {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('#').unwrap();
        let s: Vec<&str> = s.split("").collect();

        let r = i32::from_str_radix(&s[1..3].concat(), 16)?;
        let g = i32::from_str_radix(&s[3..5].concat(), 16)?;
        let b = i32::from_str_radix(&s[5..7].concat(), 16)?;
        let a = i32::from_str_radix(&s[7..9].concat(), 16)?;

        Ok(Self {
            r: (r as f32 / 255.0),
            g: (g as f32 / 255.0),
            b: (b as f32 / 255.0),
            a: (a as f32 / 255.0),
        })
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", 
        (self.r * 255.0) as i32, 
        (self.g * 255.0) as i32, 
        (self.b * 255.0) as i32, 
        (self.a * 255.0) as i32
    )}
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let value = value.trim_start_matches("#");
        let u32_value = u32::from_str_radix(value, 16).unwrap();
        Color { 
            r: (u32_value >> 24 & 0xFF) as f32 / 0xFF as f32,
            g: (u32_value >> 16 & 0xFF) as f32 / 0xFF as f32,
            b: (u32_value >> 8 & 0xFF) as f32 / 0xFF as f32,
            a: (u32_value & 0xFF) as f32 / 0xFF as f32
        }
    }
}