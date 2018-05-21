use chunk::Chunk;
use chunk::ChunkType;
use little_endian as le;
use std::string::FromUtf8Error;
use util::*;

#[derive(Clone)]
pub struct SubChunk {
    pub id: u32,
    pub data: Vec<u8>,
}

impl SubChunk {
    pub fn new(id: &str) -> Box<SubChunk> {
        Box::new(SubChunk {
            id: encode_str(id),
            data: vec![],
        })
    }

    pub fn new_str(id: &str, data: &str) -> Box<SubChunk> {
        let mut cnk = Self::new(id);
        cnk.fill_string(data);
        cnk
    }

    pub fn new_data(id: &str, data: Vec<u8>) -> Box<SubChunk> {
        let mut cnk = Self::new(id);
        cnk.data = data;
        cnk
    }

    pub fn new_u8(id: &str, data: u8) -> Box<SubChunk> {
        Self::new_data(id, vec![data])
    }

    pub fn new_u16(id: &str, data: u16) -> Box<SubChunk> {
        let mut buf = [0; 2];
        le::write(&mut buf, data);
        Self::new_data(id, vec![buf[0], buf[1]])
    }

    pub fn new_u32(id: &str, data: u32) -> Box<SubChunk> {
        Self::new_data(id, encode_u32(data))
    }

    pub fn as_str(&self) -> Result<String, FromUtf8Error> {
        self.data_string()
    }

    pub fn as_u8(&self) -> u8 {
        self.data[0]
    }

    pub fn as_u16(&self) -> u16 {
        le::read(&self.data[0..2])
    }

    pub fn as_u32(&self) -> u32 {
        le::read(&self.data[0..4])
    }
}

impl Chunk for SubChunk {
    fn as_subchunk(&self) -> Option<&SubChunk> {
        Some(self)
    }

    fn internal(&self) -> ChunkType {
        ChunkType::SubChunk(self)
    }

    fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
    fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    fn set_id(&mut self, id: &str) {
        self.id = encode_str(id);
    }

    fn total_size(&self) -> u32 {
        // we need to add an extra byte if the chunk isn't word aligned
        if self.size() % 2 == 0 {
            return self.size() + 8;
        } else {
            return self.size() + 9;
        }
    }

    fn size(&self) -> u32 {
        self.data.len() as u32
    }

    fn id(&self) -> String {
        parse_id(self.id)
    }

    fn compile(&self) -> Vec<u8> {
        let mut data = vec![];
        data.append(&mut encode_u32(self.id));
        data.append(&mut encode_u32(self.data.len() as u32));

        for byte in &self.data {
            data.push(*byte);
        }

        // add phantom byte if not word aligned
        if self.size() % 2 != 0 {
            data.push(0);
        }

        data
    }
}
