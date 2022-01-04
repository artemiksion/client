pub struct Message{
    command: u8,
    length: u32,
    data: String,
}

impl Message{
    fn to_bytes(&mut self) -> &[u8] {
        let mut v:Vec<u8> = Vec::new();
        v.push(self.command);
        v.append(self.length.to_be_bytes());
    }

    fn new(com: &u8, len: &u32, dt: &String) -> Message {
        let mes = Message {
            command: *com,
            length: *len,
            data: *dt,
        };
        return mes;
    }
}

