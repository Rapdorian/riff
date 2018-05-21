use chunk::*;
use subchunk::SubChunk;
use util::*;

pub struct List {
    pub id: u32,
    pub form: u32,
    pub data: Vec<Box<Chunk>>,
}

impl List {
    pub fn new(id: &str, form: &str) -> Box<List> {
        Box::new(List {
            id: encode_str(id),
            form: encode_str(form),
            data: vec![],
        })
    }

    pub fn set_type(&mut self, form: &str) {
        self.form = encode_str(form);
    }

    pub fn form(&self) -> String {
        parse_id(self.form)
    }

    pub fn new_subs(id: &str, form: &str, subs: Vec<Box<Chunk>>) -> Box<List> {
        let mut lst = Self::new(id, form);
        for cnk in subs {
            lst.data.push(cnk);
        }
        lst
    }

    pub fn get_subs(&self, id: &str) -> Vec<&Box<Chunk>> {
        let mut chunks = vec![];

        for chunk in &self.data {
            if chunk.id() == id { 
                chunks.push(chunk);
            }
            // check form if this is a list
            if let Some(ref list) = chunk.as_list(){
                if list.form() == id{
                    chunks.push(chunk);
                }
            }
        }
        chunks
    }
}

impl Chunk for List {
    fn as_list(&self) -> Option<&List> {
        Some(self)
    }

    fn internal(&self) -> ChunkType {
        ChunkType::List(self)
    }

    fn set_data(&mut self, data: Vec<u8>) {
        // check if we already have a 'DATA' tag
        for chunk in &mut self.data {
            if chunk.id() == "DATA" {
                chunk.set_data(data);
                return;
            }
        }
        self.data.push(Box::new(SubChunk {
            id: encode_str("DATA"),
            data: data,
        }));
    }

    fn data(&self) -> Vec<u8> {
        // check if we already have a 'DATA' tag
        for chunk in &self.data {
            if chunk.id() == "DATA" {
                return chunk.data();
            }
        }
        return vec![];
    }

    fn id(&self) -> String {
        parse_id(self.id)
    }

    fn set_id(&mut self, id: &str) {
        self.id = encode_str(id);
    }

    fn size(&self) -> u32 {
        let mut sum = 4;
        for chunk in &self.data {
            sum += chunk.total_size();
        }
        sum
    }

    fn compile(&self) -> Vec<u8> {
        let mut data = vec![];

        data.append(&mut encode_u32(self.id));
        data.append(&mut encode_u32(self.size()));
        data.append(&mut encode_u32(self.form));

        for chunk in &self.data {
            data.append(&mut chunk.compile());
        }

        data
    }
}
