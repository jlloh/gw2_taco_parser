mod trail_parser;
mod xml_parser;

fn main() {
    let folder = "data";
    let xml_file = "dw_coral.xml";

    let j = serde_json::to_string(&xml_parser::process_taco_data(folder.to_string(), xml_file.to_string())).unwrap();
    println!("{}", j);
}
 