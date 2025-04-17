use std::{fs::File, io::{BufWriter, Write as _}};
use flate2::{write::GzEncoder, Compression};

use crate::{Read, Write, Error, ReadVersioned};
#[cfg(feature = "image")]
use super::image::Image;
use super::{layer::Layer, level::Level};
use crate::types::{local_level::LocalLevel, level_data::LevelData, author_replay::AuthorReplay, theme::Theme, vec2::Vec2};
use std::path::Path;

/// A full Exoracer level.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Exolvl {
    /// The local level data for this level.
    pub local_level: LocalLevel,
    /// The actual level data.
    pub level_data: LevelData,
    /// The data for the author time replay.
    pub author_replay: AuthorReplay,
}

const EXPECTED_MAGIC: &[u8; 4] = b"NYA^";

impl Read for Exolvl {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let magic: [u8; 4] = Read::read(input)?;

        if &magic != EXPECTED_MAGIC {
            return Err(Error::WrongMagic);
        }

        let local_level = LocalLevel::read(input)?;
        let level_data = ReadVersioned::read(input, local_level.serialization_version)?;
        let author_replay = Read::read(input)?; 

        Ok(Self {
            local_level,
            level_data,
            author_replay,
        })
    }
}

impl Write for Exolvl {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        EXPECTED_MAGIC.write(output)?;
        self.local_level.write(output)?;
        self.level_data.write(output)?;
        self.author_replay.write(output)
    }
}

impl Default for Exolvl {
    fn default() -> Self {
        let level_id = uuid::Uuid::new_v4();
        let first_layer = Layer::default();
        Self { 
            local_level: LocalLevel { 
                serialization_version: 19,
                level_id: level_id.clone(),
                level_version: 1,
                level_name: "".to_string(),
                thumbnail: "".to_string(),
                creation_date: chrono::Utc::now(),
                update_date: chrono::Utc::now(),
                author_time: Default::default(),
                author_lap_times: Default::default(),
                silver_medal_time: Default::default(),
                gold_medal_time: Default::default(),
                laps: 1,
                private: Default::default(),
                nova_level: true,
            }, 
            level_data: LevelData { 
                level_id,
                level_version: 1,
                nova_level: true,
                under_decoration_tiles: Default::default(),
                background_decoration_tiles: Default::default(),
                terrain_tiles: Default::default(),
                floating_zone_tiles: Default::default(),
                object_tiles: Default::default(),
                foreground_decoration_tiles: Default::default(),
                objects: Default::default(),
                layers: vec![first_layer],
                prefabs: Default::default(),
                brushes: Default::default(),
                patterns: Default::default(),
                color_palette: Some(Default::default()),
                author_time: Default::default(),
                author_lap_times: Default::default(),
                silver_medal_time: Default::default(),
                gold_medal_time: Default::default(),
                laps: 1,
                center_camera: Default::default(),
                scripts: Default::default(),
                scripts_folders: Default::default(),
                nova_scripts: Default::default(),
                variables_folders: Default::default(),
                global_variables: Default::default(),
                theme: Theme::Mountains,
                custom_background_color: Default::default(),
                unknown1: [0; 4],
                custom_terrain_pattern_id: Default::default(),
                custom_terrain_pattern_tiling: Default::default(),
                custom_terrain_pattern_offset: Default::default(),
                custom_terrain_color: Default::default(),
                custom_terrain_secondary_color: Default::default(),
                custom_terrain_blend_mode: Default::default(),
                custom_terrain_border_color: Default::default(),
                custom_terrain_border_thickness: Default::default(),
                custom_terrain_border_corner_radius: Default::default(),
                custom_terrain_round_reflex_angles: Default::default(),
                custom_terrain_round_collider: Default::default(),
                custom_terrain_friction: Default::default(),
                default_music: true,
                music_ids: Default::default(),
                allow_direction_change: Default::default(),
                disable_replays: Default::default(),
                disable_revive_pads: Default::default(),
                disable_start_animation: Default::default(),
                gravity: Vec2 { x: 0.0.into(), y: (-75.0).into() } 
            },
            author_replay: AuthorReplay(Default::default()),
        }
    }
}

impl Exolvl {
    pub fn read_from_exolvl_file(path: &Path) -> Result<Self, Error>{
        let file = File::open(path)?;
        let buf_file = std::io::BufReader::new(file); // Add buffering here
        let mut file = flate2::read::GzDecoder::new(buf_file);
        Exolvl::read(&mut file)
    }

    pub fn write_as_exolvl_file(&mut self, path: &Path) -> Result<(), Error>{
        
        // Write self to a buffer
        let mut buf = vec![];
        {
            let mut buf_writer = BufWriter::new(&mut buf);
            self.write(&mut buf_writer)?;
            buf_writer.flush()?;
        }

        // gzip the buffer
        let file = File::create(path)?;
        let buf_file = BufWriter::new(file); // <-- Add buffering here
        let mut encoder = GzEncoder::new(buf_file, Compression::default());
        encoder.write_all(&buf)?;
        encoder.finish()?; // This will flush the BufWriter as well

        Ok(())
    }

    #[cfg(feature = "serde")]
    pub fn save_as_json_to_file(&self, dir: &Path) -> Result<File, Error> {
        let mut new_file = File::create(dir.join(format!("{}.exolvl.json", self.local_level.level_name.clone()))).unwrap();
        std::io::Write::write_all(
            &mut new_file, 
            serde_json::to_string(self).unwrap().as_bytes()
        ).unwrap();
        Ok(new_file)
    }

    #[cfg(feature = "serde")]
    pub fn  read_from_json(path: &Path) -> Result<Self, Error> {
        let read_file = File::open(path).map_err(|e| Error::FileRead(e))?;
        let reader = std::io::BufReader::new(read_file);
        let level: Exolvl = serde_json::from_reader(reader).map_err(|e| Error::SerdeParse(e))?;
        Ok(level)
    }

    #[cfg(feature = "image")]
    pub fn add_image_pattern(&mut self, path_to_image: &Path) -> Result<(), image::ImageError> {
        let img: Image = image::open(path_to_image)?.into();
        self.level_data.patterns.push(vec![img].into());
        Ok(())
    }

    pub fn from_level(level: Level, name: &str, thumbnail: &str) -> Self {
        Self { 
            local_level: LocalLevel { 
                serialization_version: level.serialization_version, 
                level_id: level.level_data.level_id, 
                level_version: level.level_data.level_version, 
                level_name: name.to_string(), 
                thumbnail: thumbnail.to_string(),
                creation_date: chrono::Utc::now(),
                update_date: chrono::Utc::now(), 
                author_time: level.level_data.author_time, 
                author_lap_times: level.level_data.author_lap_times.clone(),
                silver_medal_time: level.level_data.silver_medal_time, 
                gold_medal_time: level.level_data.gold_medal_time, 
                laps: level.level_data.laps, 
                private: true, 
                nova_level: level.level_data.nova_level
            }, 
            level_data: level.level_data, 
            author_replay: AuthorReplay(vec![]) 
        }
    }
}