use std::fs::File;
use crate::{Read, Write, Error, ReadVersioned};
use crate::types::{level_data::LevelData, theme::Theme, vec2::Vec2};
use std::path::Path;

/// A full Exoracer level.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Level {
    /// The serialization_version of this level.
    pub serialization_version: i32,
    /// The actual level data.
    pub level_data: LevelData,
}

const EXPECTED_MAGIC: &[u8; 4] = b"NYA%";

impl Read for Level {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let magic: [u8; 4] = Read::read(input)?;

        if &magic != EXPECTED_MAGIC {
            return Err(Error::WrongMagic);
        }

        let serialization_version: i32 = Read::read(input)?;

        let level_data = ReadVersioned::read(input, serialization_version)?;

        Ok(Self {
            serialization_version,
            level_data,
        })
    }
}

impl Write for Level {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        EXPECTED_MAGIC.write(output)?;
        self.serialization_version.write(output)?;
        self.level_data.write(output)
    }
}

impl Default for Level {
    fn default() -> Self {
        let level_id = uuid::Uuid::new_v4();
        Self { 
            serialization_version: 18, 
            level_data: LevelData { 
                level_id: level_id,
                level_version: 1,
                nova_level: true,
                under_decoration_tiles: Default::default(),
                background_decoration_tiles: Default::default(),
                terrain_tiles: Default::default(),
                floating_zone_tiles: Default::default(),
                object_tiles: Default::default(),
                foreground_decoration_tiles: Default::default(),
                objects: Default::default(),
                layers: Default::default(),
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
                nova_scripts: Default::default(),
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
        }
    }
}

impl Level {
    pub fn read_from_path(path: &Path) -> Result<Self, Error>{
        let file = File::open(path)?;
        let mut file = flate2::read::GzDecoder::new(file);
        Level::read(&mut file)
    }

    pub fn write_to_file(&mut self, path: &Path) -> Result<(), Error>{
        let file = File::create(path)?;
        // Weirdly enough, this isnt the same compression level as the .Level files normally are, but it still works.
        let mut gzfile = flate2::write::GzEncoder::new(file, flate2::Compression::best());
        self.write(&mut gzfile)?;
        gzfile.finish()?;
        Ok(())
    }

    #[cfg(feature = "serde")]
    pub fn save_as_json_to_file(&self, dir: &Path) -> Result<File, Error> {
        let mut new_file = File::create(dir.join(format!("{}.level.json", self.level_data.level_id.clone()))).map_err(|e| Error::FileRead(e))?;
        std::io::Write::write_all(
            &mut new_file, 
            serde_json::to_string(self).map_err(|e| Error::SerdeParse(e))?.as_bytes()
        ).unwrap();
        Ok(new_file)
    }

    #[cfg(feature = "serde")]
    pub fn  read_from_json(path: &Path) -> Result<Self, Error> {
        let read_file = File::open(path).map_err(|e| Error::FileRead(e))?;
        let reader = std::io::BufReader::new(read_file);
        let level: Level = serde_json::from_reader(reader).map_err(|e| Error::SerdeParse(e))?;
        Ok(level)
    }
}