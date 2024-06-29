use std::fs::File;
use crate::{Read, Write, Error, ReadVersioned, Vec2, Image};
use crate::types::{local_level::LocalLevel, level_data::LevelData, author_replay::AuthorReplay, theme::Theme};

/// A full Exoracer level.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
        Self { 
            local_level: LocalLevel { 
                serialization_version: 18, 
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
                gravity: Vec2 { x: 0.0, y: -75.0 } 
            },
            author_replay: AuthorReplay(Default::default()),
        }
    }
}

impl Exolvl {
    pub fn read_from_path(path: &str) -> Result<Self, Error>{
        let file = File::open(path)?;
        let mut file = flate2::read::GzDecoder::new(file);
        Exolvl::read(&mut file)
    }

    pub fn write_to_file(&mut self, path: &str) -> Result<(), Error>{
        let file = File::create(path)?;
        // Weirdly enough, this isnt the same compression level as the .exolvl files normally are, but it still works.
        let mut gzfile = flate2::write::GzEncoder::new(file, flate2::Compression::best());
        self.write(&mut gzfile)?;
        Ok(())
    }

    #[cfg(feature = "serde")]
    pub fn save_as_json_to_file(&self, dir: &str) -> Result<File, serde_json::Error> {
        let mut new_file = File::create(format!("{}/new_{}.json", dir, self.local_level.level_name.clone())).unwrap();
        std::io::Write::write_all(
            &mut new_file, 
            serde_json::to_string(self).unwrap().as_bytes()
        ).unwrap();
        Ok(new_file)
    }
    #[cfg(feature = "image")]
    pub fn add_image_pattern(&mut self, path_to_image: &str) -> Result<(), image::ImageError> {
        let img: Image = image::open(path_to_image)?.into();
        self.level_data.patterns.push(vec![img].into());
        Ok(())
    }
}