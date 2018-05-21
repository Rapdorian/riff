#[macro_use]
extern crate structopt;
extern crate riff;

use riff::Chunk;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Riff Read example", about = "An example of riff usage.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let mut f = File::open(opt.input).unwrap();

    let riff = riff::read(&mut f);

    print_chunk(&riff);
}

fn print_chunk(cnk: &Box<riff::Chunk>) {
    let inner = cnk.internal();
    inner.list(|list| {
        println!("-- {} ", list.form());
        for cnk in &list.data {
            print!("     ");
            print_chunk(cnk);
        }
    });

    inner.subchunk(|cnk| {
        let s = cnk.data_string();

        if s.is_ok() {
            let s = s.unwrap();
            if s.is_ascii(){
                println!("{}: {}", cnk.id(), s);
            }else{
                println!("{}: [utf8]", cnk.id());
            }
        } else {
            println!("{}: [bin]", cnk.id());
            // println!("{}: {:X?}", cnk.id(), cnk.data);
        }
    });
}
