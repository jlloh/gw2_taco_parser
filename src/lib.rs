mod trail_parser;
mod xml_parser;

pub fn taco_to_json_str(folder_name: String, xml_file: String) -> String {
    let parsed = xml_parser::process_taco_data(folder_name, xml_file);
    let json_string = serde_json::to_string(&parsed).unwrap();
    return json_string
}