use crate::{error::Error, types::varint::Varint, Read, Write};
use cs_datetime_parse::DateTimeCs;
use ordered_float::OrderedFloat;
use uuid::Uuid;
use glam::Vec2;

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
        Varint(u64::try_from(self.len()).unwrap()).write(output)?;

        for c in self.chars() {
            (c as u8).write(output)?;
        }

        Ok(())
    }
}

impl Write for &str {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Varint(u64::try_from(self.len()).unwrap()).write(output)?;

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
//    #[cfg_attr(
//        feature = "tracing",
//        tracing::instrument(level = "debug", name = "Vec::read", skip(input))
//    )]
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let len = usize::try_from(i32::read(input)?).unwrap();

//        #[cfg(feature = "tracing")]
//        debug!(?len);

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

impl Read for chrono::DateTime<chrono::Utc> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let ticks = i64::read(input)?;
        let parsed = DateTimeCs::from_binary(ticks).unwrap();

        let DateTimeCs::Utc(datetime) = parsed else {
            panic!("datetime must be utc")
        };

        Ok(Self::from_timestamp_nanos(
            datetime.unix_timestamp_nanos().try_into().unwrap(),
        ))
    }
}

impl Write for chrono::DateTime<chrono::Utc> {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let datetime = time::OffsetDateTime::from_unix_timestamp_nanos(
            self.timestamp_nanos_opt().unwrap().into(),
        )
        .unwrap();

        let datetime = DateTimeCs::Utc(datetime);

        datetime.to_binary().unwrap().write(output)
    }
}

impl Read for Uuid {
//    #[cfg_attr(
//        feature = "tracing",
//        tracing::instrument(level = "debug", name = "Uuid::read", skip(input))
//    )]
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self::parse_str(&String::read(input)?).unwrap())
    }
}

impl Write for Uuid {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.to_string().write(output)
    }
}

impl Read for OrderedFloat<f32> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self(f32::read(input)?))
    }
}

impl Write for OrderedFloat<f32> {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.0.write(output)
    }
}

impl Read for Vec2 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self { x: f32::read(input)?, y: f32::read(input)? })
    }
}

impl Write for Vec2 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}