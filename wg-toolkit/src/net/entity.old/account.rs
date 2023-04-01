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
