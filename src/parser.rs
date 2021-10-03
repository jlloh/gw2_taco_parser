// use nom::IResult;
use nom::number::complete::{le_i32, le_f32};
use nom::{
    IResult,
    sequence::tuple};
use nom::multi::fold_many0;


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
// 
// pub fn length_value(input: &[u8]) -> IResult<&[u8],&[u8]> {
    // let (input, length) = be_u16(input)?;
    // take(length)(input)
// }

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



// fn folder_function(mut acc: Vec<TrailCoordinates>, item: TrailCoordinates) -> mut Vec<TrailCoordinates> {
//     acc.push(item);
//     acc
// }

// pub fn test_parse(input: &[u8]) -> Result {
//     // do_parse!(input, 
//         // trailVersion: le_i32 >>
//         // mapID: le_i32 >>
//         // x: le_f32 >>
//         // y: le_f32 >>
//         // z: le_f32
//     // )
//     let (buffer, trailVersion) = le_i32(input).unwrap();
//     // let (buffer2, mapID) = le_i32(buffer).unwrap();
//     let x = 0.1;
//     let y = 0.2;
//     let z  = 0.3;

//     return  Result{
//             trailVersion,
//             mapID,
//             x,
//             y,
//             z
//         };
    
// }