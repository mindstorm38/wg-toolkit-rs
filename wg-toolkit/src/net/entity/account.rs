use super::interface::client_commands_port;
use super::Entity;


/// The account entity.
/// 
/// ID: 1
#[derive(Debug)]
pub struct Account {
    /// Part of the `AccountVersion.def` interface, just used by the 
    /// python app to check that game version is coherent.
    /// 
    /// For example `eu_1.19.1_4` as of this writing.
    pub required_version: String,
    /// The name of the account.
    pub name: String,
    
    pub initial_server_settings: Vec<u8>,
}

#[derive(Debug)]
pub enum Client {
    ClientCommandsPort(client_commands_port::Client),
}

#[derive(Debug)]
pub enum Server {
    ClientCommandsPort(client_commands_port::Server),
}

impl Entity for Account {

    type Client = Client;
    type Server = Server;

    fn decode_client<R: std::io::Read>(idx: u16, read: R) -> std::io::Result<Self::Client> {
        todo!()
    }

    fn decode_server<R: std::io::Read>(idx: u16, read: R) -> std::io::Result<Self::Server> {
        todo!()
    }

}
