#[derive(Debug, Clone, Copy)]
pub struct Message {
    pub id: u8,
    pub value: u16,
}

pub fn parse_message(buf: &[u8]) -> Result<Message, &'static str> {
    if buf.len() < 3 {
        return Err("Buffer too short");
    }

    let id = buf[0];
    let value = u16::from_be_bytes([buf[1], buf[2]]);

    Ok(Message { id, value })
}

