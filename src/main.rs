mod trail_parser;
mod xml_parser;

use std::fs::{read, read_to_string};
use std::path::Path as OsPath;


fn main() {
    let folder = "/home/jl/hdd_drive/GW2/taco_markers/";
    let xml_file = "tw_festival01_lunarnewyear.xml";
    // let folder = "data/";
    // let xml_file = "dw_coral.xml";
    // let folder = OsPath::new(&folder_name);
    let contents = read_to_string(OsPath::new(folder).join(xml_file)).unwrap();
    let xml_parsed = xml_parser::parse_xml(&contents);
    println!("{:#?}", xml_parsed.marker_category)

    // let j = serde_json::to_string(&xml_parser::process_taco_data(folder.to_string(), xml_file.to_string())).unwrap();
    // println!("{}", j);
}
 