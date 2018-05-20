use chunk::Chunk;
use util::*;

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
}

impl Chunk for SubChunk {

    fn data(&self) -> Vec<u8>{
        self.data.clone()
    }
    fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    fn set_id(&mut self, id: &str) {
        self.id = encode_str(id);
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
        data
    }
}
