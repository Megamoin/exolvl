use crate::types::layer::Layer;
use crate::types::object_id::ObjectId;
use crate::types::theme::Theme;
use crate::types::vec2::Vec2;
use crate::types::color::Color;
use crate::types::exolvl::Exolvl;
use crate::types::object_property::ObjectProperty;
use crate::types::object::Object;
// use crate::{Write as _, Read as _};
use std::{
    error::Error,
    io::BufReader
};
use image::Rgba;
use std::fs::File;
use std::io::Result as IoResult;
// use serde_json::Value;
use std::path::Path;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Shape {
    #[serde(rename = "type")]
    shape_type: u8,
    data: Vec<i32>,
    color: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct Shapes {
    shapes: Vec<Shape>,
}



pub fn import_image(file_path_json: &str, exolvl: Option<Exolvl>) -> IoResult<()> {

    let file = File::open(file_path_json)?;
    let reader = BufReader::new(file);

    // Deserialize JSON from the reader
    let shapes_collection: Shapes = serde_json::from_reader(reader)?;
    
    convert(&shapes_collection, "level.exolvl", exolvl).unwrap();

    Ok(())
}

fn convert(
    shapes_collection: &Shapes,
    level_name: &str,
    exolvl: Option<Exolvl>,
) -> Result<(), Box<dyn Error>> {
    let mut level = match exolvl {
        Some(level) => level,
        None => Exolvl::default(),
    };

    level.level_data.layers.push(Layer::default());

    process_image(&mut level, shapes_collection)?;

    update_level_properties(&mut level, level_name);

    set_theme(&mut level);

    write_level(&mut level)
}

fn process_image(
    level: &mut Exolvl,
    shapes_collection: &Shapes,
) -> Result<(), Box<dyn Error>> {


    let layer = level
        .level_data
        .layers
        .get_mut(0)
        .ok_or("level file doesn't have any layers")?;

    let mut canvas_size = (0, 0);

    for (entity_id, shape) in shapes_collection.shapes.iter().enumerate() {
        
        let obj = match shape.shape_type {
            1 => {
                // Rectangles
                let entity_id = entity_id.try_into()?;
                let object_id: ObjectId = ObjectId::SpriteSquare;
                let x_coordinate = *(shape.data.get(0).ok_or("Missing x-coordinate")? as &i32);
                let y_coordinate = *(shape.data.get(1).ok_or("Missing y-coordinate")? as &i32);
                let x_coordinate2 = *(shape.data.get(2).ok_or("Missing x-scale")? as &i32);
                let y_coordinate2 = *(shape.data.get(3).ok_or("Missing y-scale")? as &i32);
                
                if entity_id == 0 {
                    canvas_size = (x_coordinate2, y_coordinate2);
                    println!("{:?}", canvas_size);
                }
                
                // let x_coordinate = &(canvas_size.0 - x_coordinate);
                // let x_coordinate2 = &(canvas_size.0 - x_coordinate2);
                let y_coordinate = canvas_size.1 - y_coordinate;
                let y_coordinate2 = canvas_size.1 - y_coordinate2;

                let scale = Vec2 {
                    x: (x_coordinate2 as f32 - x_coordinate as f32).abs().into(),
                    y: (y_coordinate2 as f32 - y_coordinate as f32).abs().into(),
                };

                let position = Vec2 {
                    x: ((x_coordinate as f32 + x_coordinate2 as f32) / 2.0).into(),
                    y: ((y_coordinate as f32 + y_coordinate2 as f32) / 2.0).into(),
                };

                let color = shape.color.as_slice();

                if color.len() != 4 {
                    return Err("Color array must have 4 elements".into());
                }

                let rgba_color = Rgba([
                    color[0], // Red
                    color[1], // Green
                    color[2], // Blue
                    color[3], // Alpha
                ]); 

                let pixel: Rgba<u8> = rgba_color;


                get_object(entity_id, 
                    object_id, 
                    position, 
                    0.0, 
                    pixel, 
                    layer.layer_id, 
                    vec![ObjectProperty::Width(scale.x), ObjectProperty::Height(scale.y)]
                )

            },
            2 => {
                // Rotated Rectangles
                let entity_id = entity_id.try_into()?;
                let object_id: ObjectId = ObjectId::SpriteSquare;
                let x_coordinate = *(shape.data.get(0).ok_or("Missing x-coordinate")? as &i32);
                let y_coordinate = *(shape.data.get(1).ok_or("Missing y-coordinate")? as &i32);
                let x_coordinate2 = *(shape.data.get(2).ok_or("Missing x-scale")? as &i32);
                let y_coordinate2 = *(shape.data.get(3).ok_or("Missing y-scale")? as &i32);
                let mut rotation = *(shape.data.get(4).ok_or("Missing rotation")? as &i32);


                // let x_coordinate = &(canvas_size.0 - x_coordinate);
                // let x_coordinate2 = &(canvas_size.0 - x_coordinate2);
                let y_coordinate = canvas_size.1 - y_coordinate;
                let y_coordinate2 = canvas_size.1 - y_coordinate2;
                
                if rotation > 180 {
                    rotation = rotation - 180
                }
                
                rotation = 180 - rotation;


                let scale = Vec2 {
                    x: (x_coordinate2 as f32 - x_coordinate as f32).abs().into(),
                    y: (y_coordinate2 as f32 - y_coordinate as f32).abs().into(),
                };

                let position = Vec2 {
                    x: ((x_coordinate as f32 + x_coordinate2 as f32) / 2.0).into(),
                    y: ((y_coordinate as f32 + y_coordinate2 as f32) / 2.0).into(),
                };

                let color = shape.color.as_slice();

                if color.len() != 4 {
                    return Err("Color array must have 4 elements".into());
                }

                let rgba_color = Rgba([
                    color[0], // Red
                    color[1], // Green
                    color[2], // Blue
                    color[3], // Alpha
                ]); 

                let pixel: Rgba<u8> = rgba_color;


                get_object(
                    entity_id, 
                    object_id, 
                    position, 
                    rotation as f32, 
                    pixel, 
                    layer.layer_id, 
                    vec![ObjectProperty::Width(scale.x), ObjectProperty::Height(scale.y)]
                )
            },
            4 => {
                // Rectangles
                let entity_id = entity_id.try_into()?;
                let object_id: ObjectId = ObjectId::SpriteSquare;
                let x_coordinate = *(shape.data.get(0).ok_or("Missing x-coordinate")? as &i32);
                let y_coordinate = *(shape.data.get(1).ok_or("Missing y-coordinate")? as &i32);
                let x_coordinate2 = *(shape.data.get(2).ok_or("Missing x-scale")? as &i32);
                let y_coordinate2 = *(shape.data.get(3).ok_or("Missing y-scale")? as &i32);
                
                if entity_id == 0 {
                    canvas_size = (x_coordinate2, y_coordinate2);
                    println!("{:?}", canvas_size);
                }
                
                // let x_coordinate = &(canvas_size.0 - x_coordinate);
                // let x_coordinate2 = &(canvas_size.0 - x_coordinate2);
                let y_coordinate = canvas_size.1 - y_coordinate;
                let y_coordinate2 = canvas_size.1 - y_coordinate2;

                let scale = Vec2 {
                    x: (x_coordinate2 as f32 - x_coordinate as f32).abs().into(),
                    y: (y_coordinate2 as f32 - y_coordinate as f32).abs().into(),
                };

                let position = Vec2 {
                    x: ((x_coordinate as f32 + x_coordinate2 as f32) / 2.0).into(),
                    y: ((y_coordinate as f32 + y_coordinate2 as f32) / 2.0).into(),
                };

                let color = shape.color.as_slice();

                if color.len() != 4 {
                    return Err("Color array must have 4 elements".into());
                }

                let rgba_color = Rgba([
                    color[0], // Red
                    color[1], // Green
                    color[2], // Blue
                    color[3], // Alpha
                ]); 

                let pixel: Rgba<u8> = rgba_color;


                get_object(entity_id, 
                    object_id, 
                    position, 
                    0.0, 
                    pixel, 
                    layer.layer_id, 
                    vec![ObjectProperty::Width(scale.x), ObjectProperty::Height(scale.y)]
                )

            },
            8 => {
                // Ellipses
                let entity_id = entity_id.try_into()?;
                let object_id: ObjectId = ObjectId::SpriteCircle;
                let x_coordinate = *(shape.data.get(0).ok_or("Missing x-coordinate")? as &i32);
                let y_coordinate = *(shape.data.get(1).ok_or("Missing y-coordinate")? as &i32);
                let width = *(shape.data.get(2).ok_or("Missing rotation")? as &i32) * 2;
                let height = *(shape.data.get(3).ok_or("Missing rotation")? as &i32) * 2;

                // let x_coordinate = &(canvas_size.0 - x_coordinate);
                let y_coordinate = canvas_size.1 - y_coordinate;

                let position = Vec2 {
                    x: (x_coordinate as f32).into(),
                    y: (y_coordinate as f32).into(),
                };

                let color = shape.color.as_slice();

                if color.len() != 4 {
                    return Err("Color array must have 4 elements".into());
                }

                let rgba_color = Rgba([
                    color[0], // Red
                    color[1], // Green
                    color[2], // Blue
                    color[3], // Alpha
                ]); 

                let pixel: Rgba<u8> = rgba_color;

                get_object(
                    entity_id, 
                    object_id, 
                    position, 
                    0.0, 
                    pixel, 
                    layer.layer_id, 
                    vec![ObjectProperty::Width((width as f32).into()), ObjectProperty::Height((height as f32).into()), ObjectProperty::Resolution(32), ObjectProperty::TotalAngle(360.0.into())]
                )
            },
            16 => {
                // Rotated Ellipses
                let entity_id = entity_id.try_into()?;
                let object_id: ObjectId = ObjectId::SpriteCircle;
                let x_coordinate = *(shape.data.get(0).ok_or("Missing x-coordinate")? as &i32);
                let y_coordinate = *(shape.data.get(1).ok_or("Missing y-coordinate")? as &i32);
                let width = *(shape.data.get(2).ok_or("Missing rotation")? as &i32) * 2;
                let height = *(shape.data.get(3).ok_or("Missing rotation")? as &i32) * 2;
                let mut rotation = *(shape.data.get(4).ok_or("Missing rotation")? as &i32);

                // let x_coordinate = &(canvas_size.0 - x_coordinate);
                let y_coordinate = canvas_size.1 - y_coordinate;

                if rotation > 180 {
                    rotation = rotation - 180
                }
                
                rotation = 180 - rotation;

                let position = Vec2 {
                    x: (x_coordinate as f32).into(),
                    y: (y_coordinate as f32).into(),
                };

                let color = shape.color.as_slice();

                if color.len() != 4 {
                    return Err("Color array must have 4 elements".into());
                }

                let rgba_color = Rgba([
                    color[0], // Red
                    color[1], // Green
                    color[2], // Blue
                    color[3], // Alpha
                ]); 

                let pixel: Rgba<u8> = rgba_color;

                get_object(
                    entity_id, 
                    object_id, 
                    position, 
                    rotation as f32,
                    pixel, 
                    layer.layer_id, 
                    vec![ObjectProperty::Width((width as f32).into()), ObjectProperty::Height((height as f32).into()), ObjectProperty::Resolution(32), ObjectProperty::TotalAngle(360.0.into())]
                )
            },
            32 => {
                // Circles
                let entity_id = entity_id.try_into()?;
                let object_id: ObjectId = ObjectId::SpriteCircle;
                let x_coordinate = *(shape.data.get(0).ok_or("Missing x-coordinate")? as &i32);
                let y_coordinate = *(shape.data.get(1).ok_or("Missing y-coordinate")? as &i32);
                let radius = *(shape.data.get(2).ok_or("Missing rotation")? as &i32);

                // let x_coordinate = &(canvas_size.0 - x_coordinate);
                let y_coordinate = canvas_size.1 - y_coordinate;

                let position = Vec2 {
                    x: (x_coordinate as f32).into(),
                    y: (y_coordinate as f32).into(),
                };

                let color = shape.color.as_slice();

                if color.len() != 4 {
                    return Err("Color array must have 4 elements".into());
                }

                let rgba_color = Rgba([
                    color[0], // Red
                    color[1], // Green
                    color[2], // Blue
                    color[3], // Alpha
                ]); 

                let pixel: Rgba<u8> = rgba_color;
                
                get_object(
                    entity_id, 
                    object_id, 
                    position, 
                    0.0, 
                    pixel, 
                    layer.layer_id, 
                    vec![ObjectProperty::Width((radius as f32).into()), ObjectProperty::Height((radius as f32).into()), ObjectProperty::Resolution(32), ObjectProperty::TotalAngle(360.0.into())]
                )
            }
            _ => {panic!("this type has not been implemented yet!")}
        };
        
        level.level_data.objects.push(obj);

        layer.children.push(entity_id.try_into()?);
        
    }

    Ok(())
}

fn get_object(entity_id: i32, object_id: ObjectId, position: Vec2, rotation: f32, pixel: Rgba<u8>, layer_id: i32, mut properties: Vec<ObjectProperty>) -> Object {
    
    properties.push(ObjectProperty::Color(Color {
        r: (pixel.0[0] as f32 / 255.0).into(),
        g: (pixel.0[1] as f32 / 255.0).into(),
        a: (pixel.0[3] as f32 / 255.0).into(),
        b: (pixel.0[2] as f32 / 255.0).into(),
    }));

    
    Object {
        entity_id,
        object_id,
        prefab_entity_id: 0,
        prefab_id: 0,
        position,
        scale: Vec2 { x: 1.0.into(), y: 1.0.into() },
        rotation: rotation.into(),
        tag: String::new(),
        properties: properties,
        in_layer: layer_id,
        in_group: 0,
        group_members: vec![],
    }
}

fn set_theme(level: &mut Exolvl) {
    level.level_data.theme = Theme::Custom;

    level.level_data.custom_terrain_color = Color {
        r: 1.0.into(),
        g: 1.0.into(),
        b: 1.0.into(),
        a: 1.0.into(),
    };

    level.level_data.custom_terrain_border_color = Color {
        r: 1.0.into(),
        g: 1.0.into(),
        b: 1.0.into(),
        a: 1.0.into(),
    };

    level.level_data.custom_background_color = Color {
        r: 0.0.into(),
        g: 0.0.into(),
        b: 0.0.into(),
        a: 1.0.into(),
    };
}

fn update_level_properties(level: &mut Exolvl, level_name: &str) {
    let created_time = chrono::Utc::now();

    level.local_level.level_id = Uuid::new_v4();
    level.local_level.level_name = level_name.to_string();
    level.local_level.creation_date = created_time;
    level.local_level.update_date = created_time;
}

fn write_level(level: &mut Exolvl) -> Result<(), Box<dyn Error>> {    
    Exolvl::write_to_file(level, Path::new("/Users/florian/Desktop/Shape_Test.exolvl"))?;
    Ok(())
}