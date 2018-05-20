use little_endian as le;
use std::io::Read;

pub fn encode_u32(x: u32) -> Vec<u8> {
    let mut buf = [0; 4];
    le::write(&mut buf, x);
    vec![buf[0], buf[1], buf[2], buf[3]]
}

pub fn parse_id(x: u32) -> String {
    let buf = encode_u32(x);
    format!("{}{}{}{}", buf[0] as char, buf[1] as char, buf[2] as char, buf[3] as char)
}

pub fn encode_str(s: &str) -> u32 {
    let mut chars = s.chars();
    let mut id = vec![];

    while id.len() < 4 {
        let c = chars.next().unwrap();
        if c.is_ascii() {
            id.push(c as u8);
        }
    }
    le::read(&id.as_slice())
}

pub fn parse_u32<F: Read>(file: &mut F) -> u32 {
    let mut bytes = [0; 4];
    file.read_exact(&mut bytes).unwrap();
    le::read(&bytes)
}

pub fn read_byte<F: Read>(file: &mut F) -> u8 {
    let mut buf = [0];
    file.read_exact(&mut buf).unwrap();
    buf[0]
}
