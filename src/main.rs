use crate::parser::{parse_trail, process_taco_data};
use std::fs::File;
use std::io::prelude::*;
use serde_json::Result;

pub mod parser;

fn main() {
    let aaa = include_bytes!("trail_bloodstone_fen_1.trl");
    let result = parse_trail(aaa).unwrap();
    // println!("{:?}", result.1);
    let contents = include_str!("dw_coral.xml");
    let xml_parsed = parser::parse_xml(&contents);
    // println!("{:#?}", xml_parsed);
    // println!("{:#?}", process_taco_data(xml_parsed));

    let j = serde_json::to_string(&process_taco_data(xml_parsed)).unwrap();

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
}
 