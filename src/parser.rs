// use nom::IResult;
use nom::number::complete::{le_i32, le_f32};
use nom::{
    IResult,
    sequence::tuple};
use nom::multi::fold_many0;
use serde::Deserialize;
use serde_xml_rs;
use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub struct Trail {
    trail_version: i32,
    map_id: i32,
    coordinates: Vec<TrailCoordinates>
}

#[derive(Debug,PartialEq)]
pub struct TrailCoordinates {
    x: f32,
    y: f32,
    z: f32
}

fn parse_coordinates(input: &[u8]) -> IResult<&[u8], TrailCoordinates> {
    let (input, (x, y, z)) = tuple((le_f32, le_f32, le_f32))(input)?;
    Ok((input, TrailCoordinates{x, y, z}))
}

pub fn parse_trail(input: &[u8]) -> IResult<&[u8], Trail> {
    // Example usage of nom
    // let (input, (x, y, z)) = tuple((le_f32, le_f32, le_f32))(input)?;
    // let (input, x) = parse_coordinates(input)?;

    let folder_function = |mut acc: Vec<TrailCoordinates>, item: TrailCoordinates| {
        acc.push(item);
        acc
    };
    let (input, (trail_version, map_id)) = tuple((le_i32, le_i32))(input)?;
    let mut parse_all_coordinates = fold_many0(parse_coordinates, Vec::new, folder_function);
    let (input,vec ) = parse_all_coordinates(input)?;
    Ok((input, Trail{
        trail_version,
        map_id,
        coordinates: vec
    }))
}

#[derive(Debug, Deserialize)]
pub struct OverlayData {
    #[serde(rename = "MarkerCategory")]
    marker_category: MarkerCategory,
    #[serde(rename = "POIs")]
    pois: POIArrayContainer,
}

#[derive(Debug, Deserialize)]
pub struct MarkerCategoryArrayContainer {
    name: String,
    #[serde(rename = "$value")]
    marker_category_array: Vec<MarkerCategory>,
}

#[derive(Debug, Deserialize)]
pub struct MarkerCategory {
    name: String,
    #[serde(rename = "iconFile")]
    icon_file: Option<String>,
    #[serde(rename = "MarkerCategory")]
    marker_category: Option<MarkerCategoryArrayContainer>,
    #[serde(rename = "heightOffset")]
    height_offset: Option<f32>
}

#[derive(Debug, Deserialize)]
pub struct POIArrayContainer {
    #[serde(rename = "$value")]
    poi_array: Vec<PoiItems>,
}

#[derive(Debug, Deserialize)]
struct POI {
    #[serde(rename = "MapID")]
    map_id: u32,
    xpos: f32,
    ypos: f32,
    zpos: f32,
    #[serde(rename = "iconFile")]
    icon_file: Option<String>,
    #[serde(rename = "heightOffset")]
    height_offset: Option<f32>
}

#[derive(Debug, Deserialize)]
struct TrailMetadata {
    #[serde(rename = "trailData")]
    trail_data: String,
    texture: String,
    color: String,
    #[serde(rename = "iconFile")]
    icon_file: Option<String>
}

#[derive(Debug, Deserialize)]
enum PoiItems {
    POI(POI),
    #[serde(rename = "Trail")]
    TrailMetadata(TrailMetadata)
}

//https://docs.rs/serde-xml-rs/0.5.1/serde_xml_rs/
// TODO: Accept Trail by making it an enum like the example
pub fn parse_xml(xml: &str ) -> OverlayData {
    //let root: minidom::Element = xml.parse().unwrap();
    //println!("{:#?}", root);
    // println!("{:?}", xml);
    let data: OverlayData = serde_xml_rs::from_str(xml).unwrap();
    data
}

pub fn process_taco_data(input: OverlayData) -> HashMap<u32, Vec<[f32; 3]>> {
    let root_marker_category = input.marker_category;

    let marker_category_array_container = root_marker_category.marker_category.unwrap();

    let marker_category_array = marker_category_array_container.marker_category_array;

    let poi_array = input.pois.poi_array;

    let folder_function = | mut acc: HashMap<u32, Vec<[f32; 3]>>, item: &PoiItems | {
        match item {
            PoiItems::POI(poi)=> { 
                let map_id = poi.map_id;
                let array_ = acc.get(&map_id);
                match array_ {
                    Some(vec_) => {
                        let mut res = vec_.clone();
                        // res.push(xpos);
                        res.push([poi.xpos, poi.ypos, poi.zpos]);
                        acc.insert(map_id, res);
                    }
                    _ => {
                        let mut vec_ = Vec::new();
                        vec_.push([poi.xpos, poi.ypos, poi.zpos]);
                        acc.insert(map_id, vec_);
                    }
                }
            }
            PoiItems::TrailMetadata(trail_metadata) => {
                let z = &trail_metadata.trail_data;
            }
        }
        return acc;
    };

    let map_name: HashMap<_, _> = HashMap::new();
    let result = poi_array.iter().fold(map_name, folder_function);
    return result
}

struct Converted {
    // icons: Vec<String>,
    paths: Vec<Path>
}

struct Path {
    texture: String,
    points: Vec<[f32; 3]>
}

// Convert to map{ map_id: 
    // {
    //     "1330": {
    //         "icons": [
    //             {
    //                 "position": [
    //                     -246.114,
    //                     19.9021,
    //                     -586.05
    //                 ],
    //                 "texture": "data/coral.png"
    //             },]
    //          "paths": [{"texture": "", points: [[1, 2, 3], ...]}]