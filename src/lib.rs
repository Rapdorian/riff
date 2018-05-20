extern crate little_endian;

use std::io::Read;
use std::io::Write;

pub const RIFF_ID: u32 = 0x46464952;
pub const LIST_ID: u32 = 0x5453494c;

pub mod util;
pub mod chunk;
pub mod subchunk;
pub mod list;

pub use util::*;
pub use chunk::*;
pub use subchunk::*;
pub use list::*;

pub fn read<F: Read>(file: &mut F) -> Box<Chunk>{
    parse_chunk(file)
}

pub fn write<F: Write>(chunk: &List, file: &mut F){
    let bytes = chunk.compile();
    file.write(bytes.as_slice());
}

fn parse_chunk<F: Read>(file: &mut F) -> Box<Chunk> {
    let id = parse_u32(file);
    let size = parse_u32(file);

    if id == RIFF_ID || id == LIST_ID {
        let mut list = List{
            id: id,
            form: parse_u32(file),
            data: vec![],
        };

        while list.size() + (list.data.len() as u32 * 8)  < size{
            list.data.push(parse_chunk(file));
        }
        return Box::new(list); 

    } else {
        let mut data = vec![]; 
        data.reserve_exact(size as usize);
        for _ in 0..size{
            data.push(read_byte(file));
        }
        let chunk = SubChunk{
            id: id,
            data: data,
        };
        return Box::new(chunk); 
    }
}
