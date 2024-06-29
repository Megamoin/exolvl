use crate::{Read, Write, Error, Uuid};
use crate::types::color::Color;

// todo: implement an enum that holds all possible names for the Properties (and maybe the type of the value as well)
// the value is stored as a string and would need to be parsed into that defined type then. Ill do it later :kappa:
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum ObjectTileProperty {
    Impulse(f32),
    Rotation(f32),
    Size(f32),
    Direction(i32),
    Bounce(bool),
    ActivationCount(i32),
    StaticPos(bool),
    StaticScale(bool),
    StaticRotation(bool),
    StaticColor(bool),
    StartScript(Uuid),
    ActivationScript(Uuid),
    LinkedObjects(Vec<i32>),
    RestoreVelocity(bool),
    ReverseDirection(bool),
    Sprite(String),
    Color(Color),
    BlendMode(String),
    ScaleX(f32),
    ScaleY(f32),
    FlipX(bool),
    FlipY(bool),
    Layer(String),
    OrderInLayer(i32),
    Text(String),
    Width(f32),
    Height(f32),
    FontSize(f32),
    PhysicsType(String),
    Health(f32),
    Killer(bool),
    DamageFromJump(bool),
    DamageFromDash(bool),
    ReverseDirOnDmg(bool),
    Floating(bool),
    FeetOffset(f32),
    DeathScript(Uuid),
    Tag(String),
    AllPlayers(bool),
    Radius(f32),
}

fn vec_i32_from_str(s: &str) -> Vec<i32> {
    s.split(",").map(|s| s.to_string().parse::<i32>().unwrap()).collect()
}
fn vec_i32_to_str(v: &Vec<i32>) -> String {
    let mut s: String = String::new();
    for i in v {
        s.push_str(&i.to_string());
        s.push(',');
    };
    s
}


impl Read for ObjectTileProperty {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
        where
            Self: Sized {
        let name: &str = &String::read(input)?;
        let value = String::read(input)?;
        
        let property = match name {
            "impulse" => Self::Impulse(value.parse().unwrap()),
            "rotation" => Self::Rotation(value.parse().unwrap()),
            "size" => Self::Size(value.parse().unwrap()),
            "direction" => Self::Direction(value.parse().unwrap()),
            "bounce" => Self::Bounce(value.parse().unwrap()),
            "activationCount" => Self::ActivationCount(value.parse().unwrap()),
            "staticPos" => Self::StaticPos(value.parse().unwrap()),
            "staticScale" => Self::StaticScale(value.parse().unwrap()),
            "staticRotation" => Self::StaticRotation(value.parse().unwrap()),
            "staticColor" => Self::StaticColor(value.parse().unwrap()),
            "startScript" => Self::StartScript(value.parse().unwrap()),
            "activationScript" => Self::ActivationScript(value.parse().unwrap()),
            "linkedObjects" => Self::LinkedObjects(vec_i32_from_str(&value)),
            "restoreVelocity" => Self::RestoreVelocity(value.parse().unwrap()),
            "reverseDirection" => Self::ReverseDirection(value.parse().unwrap()),
            "sprite" => Self::Sprite(value.parse().unwrap()),
            "color" => Self::Color(value.parse().unwrap()),
            "blendMode" => Self::BlendMode(value.parse().unwrap()),
            "scaleX" => Self::ScaleX(value.parse().unwrap()),
            "scaleY" => Self::ScaleY(value.parse().unwrap()),
            "flipX" => Self::FlipX(value.parse().unwrap()),
            "flipY" => Self::FlipY(value.parse().unwrap()),
            "layer" => Self::Layer(value.parse().unwrap()),
            "orderInLayer" => Self::OrderInLayer(value.parse().unwrap()),
            "text" => Self::Text(value.parse().unwrap()),
            "width" => Self::Width(value.parse().unwrap()),
            "height" => Self::Height(value.parse().unwrap()),
            "fontSize" => Self::FontSize(value.parse().unwrap()),
            "physicsType" => Self::PhysicsType(value.parse().unwrap()),
            "health" => Self::Health(value.parse().unwrap()),
            "killer" => Self::Killer(value.parse().unwrap()),
            "damageFromJump" => Self::DamageFromJump(value.parse().unwrap()),
            "damageFromDash" => Self::DamageFromDash(value.parse().unwrap()),
            "reverseDirOnDmg" => Self::ReverseDirOnDmg(value.parse().unwrap()),
            "floating" => Self::Floating(value.parse().unwrap()),
            "feetOffset" => Self::FeetOffset(value.parse().unwrap()),
            "deathScript" => Self::DeathScript(value.parse().unwrap()),
            "tag" => Self::Tag(value.parse().unwrap()),
            "allPlayers" => Self::AllPlayers(value.parse().unwrap()),
            "radius" => Self::Radius(value.parse().unwrap()),
            &_ => panic!(),
        };
        
        Ok(property)
    }
}

impl Write for ObjectTileProperty {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Impulse(x) => {
                "impulse".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Rotation(x) => {
                "rotation".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Size(x) => {
                "size".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Direction(x) => {
                "direction".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Bounce(x) => {
                "bounce".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::ActivationCount(x) => {
                "activationCount".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::StaticPos(x) => {
                "staticPos".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::StaticScale(x) => {
                "staticScale".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::StaticRotation(x) => {
                "staticRotation".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::StaticColor(x) => {
                "staticColor".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::StartScript(x) => {
                "startScript".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::ActivationScript(x) => {
                "activationScript".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::LinkedObjects(x) => {
                "linkedObjects".to_string().write(output)?;
                vec_i32_to_str(x).write(output)
            }
            Self::RestoreVelocity(x) => {
                "restoreVelocity".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::ReverseDirection(x) => {
                "reverseDirection".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Sprite(x) => {
                "sprite".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Color(x) => {
                "color".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::BlendMode(x) => {
                "blendMode".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::ScaleX(x) => {
                "scaleX".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::ScaleY(x) => {
                "scaleY".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::FlipX(x) => {
                "flipX".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::FlipY(x) => {
                "flipY".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Layer(x) => {
                "layer".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::OrderInLayer(x) => {
                "orderInLayer".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Text(x) => {
                "text".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Width(x) => {
                "width".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Height(x) => {
                "height".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::FontSize(x) => {
                "fontSize".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::PhysicsType(x) => {
                "physicsType".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Health(x) => {
                "health".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Killer(x) => {
                "killer".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::DamageFromJump(x) => {
                "damageFromJump".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::DamageFromDash(x) => {
                "damageFromDash".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::ReverseDirOnDmg(x) => {
                "reverseDirOnDmg".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Floating(x) => {
                "floating".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::FeetOffset(x) => {
                "feetOffset".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::DeathScript(x) => {
                "deathScript".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Tag(x) => {
                "tag".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::AllPlayers(x) => {
                "allPlayers".to_string().write(output)?;
                x.to_string().write(output)
            }
            Self::Radius(x) => {
                "radius".to_string().write(output)?;
                x.to_string().write(output)
            }
        }
    }
}