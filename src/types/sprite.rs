use strum::{Display as StrumDisplay, EnumIter, ParseError};
use strum_macros::EnumString;
use crate::traits::{Read, Write};
use crate::error::Error;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(StrumDisplay)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sprite {
    #[strum(serialize = "skins#{0}")]
    Skin(Skin),
}

impl Read for Sprite {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
    Self: Sized {
        let string = String::read(input)?;
        string.parse::<Sprite>().map_err( |e| Error::StrumParse(e))
    }
}

impl Write for Sprite {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.to_string().write(output)
    }
    
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
#[derive(EnumString, StrumDisplay, EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[strum(serialize_all = "snake_case")]
pub enum Skin{
    Alice,
    Boby,
    #[default]
    Default,
    Follower,
    Generic,
    #[strum(serialize = "ghost_1")]
    Ghost1,
    #[strum(serialize = "ghost_2")]
    Ghost2,
    #[strum(serialize = "ghost_3")]
    Ghost3,
    #[strum(serialize = "ghost_4")]
    Ghost4,
    Green,
    Jenny,
    Mark,
    OgPink,
    OgPurple,
    OgWhite,
    OgYellow,
    Red,
    Alien,
    Anniversary,
    #[strum(serialize = "anniversary_2")]
    Anniversary2,
    Artist,
    BadCap,
    BeeCostume,
    BlackCrown,
    BlackDog,
    Block,
    BlueDinosaur,
    BowTie,
    Builder,
    BunnyCostume,
    CarrotCostume,
    Cat,
    Clown,
    CoolCap,
    CountryAu,
    CountryBe,
    CountryCa,
    CountryCh,
    CountryCz,
    CountryDe,
    CountryEs,
    CountryFi,
    CountryFr,
    CountryGb,
    CountryGr,
    CountryHr,
    CountryHu,
    CountryIl,
    CountryIt,
    CountryNl,
    CountryNo,
    CountryPl,
    CountryRo,
    CountryRu,
    CountrySe,
    CountrySi,
    CountryUs,
    CuriousBot,
    Parrot,
    Stinger,
    Vampire,
    Wizard,
    Alfie,
    Cow,
    Cubix,
    #[strum(serialize = "cyborg_1")]
    Cyborg1,
    #[strum(serialize = "cyborg_2")]
    Cyborg2,
    #[strum(serialize = "cyborg_3")]
    Cyborg3,
    Daisy,
    Demon,
    Detective,
    Dog,
    Edm,
    Elf,
    Eliminated,
    EvilBot,
    FestiveLights,
    Fossil,
    Fries,
    Ghost,
    GiftBox,
    Gingerbread,
    GreenSpikes,
    GreyCat,
    GrimReaper,
    Headset,
    HelmetBanana,
    Joker,
    King,
    LabApprentice,
    Laurel,
    LunarDragon,
    MadScientist,
    Monk,
    Murica,
    Narwhal,
    Ninja,
    Nutcracker,
    Panda,
    Reindeer,
    Yeti,
    Angel,
    Astronaut,
    Bandit,
    Burger,
    Burglar,
    CardboardBox,
    Cowboy,
    Devil,
    Dinosaur,
    Fairy,
    Firefighter,
    Graffiti,
    GreenCocoa,
    Hamster,
    Jailbreak,
    Leprechaun,
    Mecha,
    Monkey,
    Mummy,
    Orca,
    PartyHat,
    Peach,
    Pirate,
    PolarBear,
    Popcorn,
    Pumpkin,
    Rabbit,
    Ramen,
    RedCocoa,
    Rplace,
    RuralFella,
    ScienceAdept,
    ScubaDiver,
    SecurityBot,
    Shades,
    Shape,
    SharkAttack,
    Siren,
    Snowman,
    Spikes,
    Spy,
    Steampunk,
    Sushi,
    TeaCup,
    Tiger,
    Toaster,
    TopHat,
    Trapper,
    TreasureChest,
    Unicorn,
    Whale,
    WhiteCrown,
    WhiteDemon,
    Wolf,
    YellowCrown,
}

use std::str::FromStr;
impl FromStr for Sprite {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("skins#") {
            let skin_str = &s[6..];
            match Skin::from_str(skin_str) {
                Ok(skin) => Ok(Sprite::Skin(skin)),
                Err(_) => Err(ParseError::VariantNotFound),
            }
        } else {
            Err(ParseError::VariantNotFound)
        }
    }
}