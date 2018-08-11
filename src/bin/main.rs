extern crate md2;
use md2::*;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("./etc/ogro/ogro.md2").expect("to open file");
    let mut reader = BufReader::new(file);

    let model = model::Model::load(&mut reader).expect("no errors");
    println!("{}", model);
}
