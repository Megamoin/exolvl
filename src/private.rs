use crate::types::{
    author_replay::AuthorReplay,
    brush::{Brush, BrushGrid, BrushObject},
    color::Color,
    exolvl::Exolvl,
    level::Level,
    image::Image,
    layer::Layer,
    level_data::LevelData,
    local_level::LocalLevel,
    nova_script::{
        action::Action, activator::Activator, nova_value::NovaValue, parameter::Parameter,
        static_type::StaticType, variable::Variable, NovaScript, function_call::{CallParameter, FunctionCall},
        dynamic_type::DynamicType, action_type::ActionType,
    },
    object::Object,
    object_property::ObjectProperty,
    old_script::{old_action::OldAction, old_action_property::OldActionProperty, old_action_type::OldActionType, Script},
    old_editor_types::{object_tile::ObjectTile, object_tile_property::ObjectTileProperty, simple_tile::SimpleTile},
    pattern::Pattern,
    prefab::Prefab,
    theme::Theme,
    varint::Varint,
    vec2::Vec2,
    object_id::ObjectId,
};
use chrono::{DateTime, Utc};
#[cfg(feature = "image")]
use image::{DynamicImage, RgbaImage};
use ordered_float::OrderedFloat;
use uuid::Uuid;
use crate::IVec2;

pub trait Sealed {}

macro_rules! impl_sealed {
    ($($ty:ty),*$(,)?) => {
        $(
            impl Sealed for $ty {}
        )*
    };
}

impl_sealed!(
    Varint,
    String,
    &str,
    u32,
    i32,
    i64,
    f32,
    bool,
    u8,
    Exolvl,
    Level,
    LocalLevel,
    DateTime<Utc>,
    LevelData,
    Pattern,
    Prefab,
    Image,
    Layer,
    Vec2,
    IVec2,
    Color,
    AuthorReplay,
    Object,
    ObjectProperty,
    Brush,
    BrushObject,
    BrushGrid,
    Script,
    NovaScript,
    OldAction,
    OldActionType,
    OldActionProperty,
    Action,
    ActionType,
    NovaValue,
    DynamicType,
    FunctionCall,
    CallParameter,
    Variable,
    StaticType,
    Activator,
    Parameter,
    Uuid,
    Theme,
    OrderedFloat<f32>,
    ObjectId,
    ObjectTile,
    ObjectTileProperty,
    SimpleTile,
    glam::Vec2
);

#[cfg(feature = "image")]
impl Sealed for DynamicImage {}

#[cfg(feature = "image")]
impl Sealed for RgbaImage {}

impl<T> Sealed for Vec<T> {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}
impl<T> Sealed for Option<T> {}