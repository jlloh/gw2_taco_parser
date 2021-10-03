use crate::parser::parse_trail;

pub mod parser;

fn main() {
    let aaa = include_bytes!("trail_bloodstone_fen_1.trl");
    let result = parse_trail(aaa).unwrap();
    println!("{:?}", result.1);
}
