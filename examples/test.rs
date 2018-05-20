extern crate riff;

use std::fs::File;
use std::io::Write;
use riff::Chunk;
use std::borrow::Borrow;

fn main(){
    // create a riff file
    let mut a = riff::SubChunk::new("ALVL");
    let mut b = riff::SubChunk::new("BLVL");

    a.fill_string("Hello World!");
    b.fill_string("This is a test");

    let mut riff = riff::List::new("RIFF", "TEST");
    riff.data.push(a);
    riff.data.push(b);

    riff.fill_string("THIS WAS ADDED DIRECTLY TO RIFF_LIST");

    // open file
    let mut f = File::create("out.riff").unwrap();

    // riff::write::<File, riff::List>(riff.borrow(), &mut f);
    riff::write(riff.borrow(), &mut f);
}
