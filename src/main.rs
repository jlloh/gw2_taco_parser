use std::path::Path;
use std::fs::read_to_string;

mod trail_parser;
mod xml_parser;

fn main() {
    // let aaa = include_bytes!("trail_bloodstone_fen_1.trl");
    // let result = trail_parser::parse_trail(aaa).unwrap();
    // println!("{:?}", result.1);
    let folder = "data";
    let xml_file = "dw_coral.xml";
    let contents = read_to_string(Path::new(folder).join(xml_file)).unwrap();
    let xml_parsed = xml_parser::parse_xml(&contents);
    // println!("{:#?}", xml_parsed);
    // println!("{:#?}", process_taco_data(xml_parsed));

    let j = serde_json::to_string(&xml_parser::process_taco_data(Path::new(folder), xml_parsed)).unwrap();

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
}
 