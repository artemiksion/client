mod ErrorClient;
use std::net::TcpStream;



pub struct Client{
    client: TcpStream,
}

impl Client{
    pub fn send_msg(&self) -> Result<(), ErrorClient::ErrorTcp> {
        Ok(())
    }

    pub fn get_msg(&self) -> Result<(), ErrorClient::ErrorTcp> {
        Ok(())
    }

    pub fn inicializate(adress :&str) -> Client {
        let s = Client {
            client: TcpStream::connect(adress).unwrap(),
        };
        return s;
    }
}