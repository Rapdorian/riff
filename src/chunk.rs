use list::List;
use subchunk::SubChunk;
use std::string::FromUtf8Error;

pub enum ChunkType<'a> {
    SubChunk(&'a SubChunk),
    List(&'a List),
}

impl<'a> ChunkType<'a> {
    pub fn list<F>(&self, mut f: F)
    where
        F: FnMut(&List),
    {
        if let ChunkType::List(ref list) = *self {
            f(list);
        }
    }

    pub fn subchunk<F>(&self, mut f: F)
    where
        F: FnMut(&SubChunk),
    {
        if let ChunkType::SubChunk(ref chunk) = *self {
            f(chunk);
        }
    }
}

pub trait Chunk {
    fn size(&self) -> u32;
    fn total_size(&self) -> u32 {
        self.size() + 8
    }
    fn compile(&self) -> Vec<u8>;

    fn set_data(&mut self, Vec<u8>);

    fn data(&self) -> Vec<u8>;

    fn id(&self) -> String;

    fn internal<'a>(&'a self) -> ChunkType<'a>;

    /// Only uses the first four characters and will skip non-ASCII characters
    fn set_id(&mut self, id: &str);

    /// Sets data to a unicode string
    fn fill_string(&mut self, data: &str) {
        self.set_data(data.bytes().collect())
    }

    /// Get data as a unicode string
    fn data_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data())
    }
}
