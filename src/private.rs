use chrono::{DateTime, Utc};
use uuid::Uuid;
#[cfg(feature = "image")]
use image::DynamicImage;

pub trait Sealed {}

macro_rules! impl_sealed {
    ($($ty:ty),*$(,)?) => {
        $(
            impl Sealed for $ty {}
        )*
    };
}

impl_sealed!(
    super::Varint,
    String,
    u32,
    i32,
    i64,
    f32,
    bool,
    u8,
    super::types::exolvl::Exolvl,
    super::types::local_level::LocalLevel,
    DateTime<Utc>,
    super::types::level_data::LevelData,
    super::types::pattern::Pattern,
    super::types::prefab::Prefab,
    super::Image,
    super::types::layer::Layer,
    super::Vec2,
    super::IVec2,
    super::types::color::Color,
    super::types::author_replay::AuthorReplay,
    super::types::simple_tile::SimpleTile,
    super::types::object_tile::ObjectTile,
    super::types::object_tile_property::ObjectTileProperty,
    super::types::object::Object,
    super::types::object_id::ObjectId,
    super::types::object_property::ObjectProperty,
    super::types::brush::Brush,
    super::types::brush_object::BrushObject,
    super::types::brush_grid::BrushGrid,
    super::types::script::Script,
    super::types::nova_script::NovaScript,
    super::types::old_action::OldAction,
    super::types::old_actiontype::OldActionType,
    super::types::old_action_property::OldActionProperty,
    super::types::action::Action,
    super::types::action_type::ActionType,
    super::types::nova_value::NovaValue,
    super::types::dynamic_type::DynamicType,
    super::types::function_call::FunctionCall,
    super::types::call_parameter::CallParameter,
    super::types::variable::Variable,
    super::types::static_type::StaticType,
    super::types::activator::Activator,
    super::types::parameter::Parameter,
    super::types::theme::Theme,
    Uuid,
    std::time::Duration,
);

#[cfg(feature = "image")]
impl Sealed for DynamicImage {}

impl<T> Sealed for Vec<T> {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}
impl<T> Sealed for Option<T> {}

