extern crate tomboy_toml_dom;

mod toy_box;

use crate::toy_box::Position;
use tomboy_toml_dom::Toml;

fn main() {
    println!("Hello, world!");

    let mut pos = Position::default();

    let doc = Toml::from_file("./config.toml");

    pos.table.width = doc.get_usize_by_key("table-width").unwrap();
    pos.table.height = doc.get_usize_by_key("table-height").unwrap();

    pos.print();
}
