extern crate tomboy_toml_dom;

mod toy_box;

use crate::toy_box::Position;
use tomboy_toml_dom::Toml;

fn main() {
    println!("Hello, world!");

    let mut pos = Position::default();

    let doc = Toml::from_file("./config.toml");

    pos.table.set_size(
        doc.get_usize_by_key_v2("table-width").unwrap().unwrap(),
        doc.get_usize_by_key_v2("table-height").unwrap().unwrap(),
    );
    pos.table.column_labels = doc
        .get_string_array_by_key("column-labels")
        .unwrap()
        .unwrap();

    pos.print();
}
