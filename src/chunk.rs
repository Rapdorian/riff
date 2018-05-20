pub trait Chunk {
    fn size(&self) -> u32;
    fn total_size(&self) -> u32 {
        self.size() + 8
    }
    fn compile(&self) -> Vec<u8>;

    fn set_data(&mut self, Vec<u8>);

    fn data(&self) -> Vec<u8>;

    fn id(&self) -> String;

    /// Only uses the first four characters and will skip non-ASCII characters
    fn set_id(&mut self, id: &str);

    /// Only works on list types
    fn change_type(&mut self, form: &str) {}

    /// Sets data to a unicode string
    fn fill_string(&mut self, data: &str) {
        self.set_data(data.bytes().collect())
    }

    /// Get data as a unicode string
    fn data_string(&self) -> String{
        String::from_utf8(self.data()).unwrap()
    }
}
