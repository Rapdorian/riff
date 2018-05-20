use chunk::Chunk;
use chunk::ChunkType;
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
}

impl Chunk for SubChunk {

    fn internal(&self) -> ChunkType{
        ChunkType::SubChunk(self)
    }

    fn data(&self) -> Vec<u8>{
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
            return self.size() + 8
        }else{
            return self.size() + 9
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
        if self.size() % 2 != 0{
            data.push(0);
        }

        data
    }
}
