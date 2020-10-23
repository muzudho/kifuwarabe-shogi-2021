extern crate tomboy_toml_dom;

mod toy_box;

use crate::toy_box::Position;
use tomboy_toml_dom::Toml;

fn main() {
    println!("Hello, world!");

    let mut pos = Position::default();

    let doc = Toml::from_file("./config.toml");

    pos.board.width = doc.get_usize_by_key("board-width").unwrap();
    pos.board.height = doc.get_usize_by_key("board-height").unwrap();

    pos.print();
}
