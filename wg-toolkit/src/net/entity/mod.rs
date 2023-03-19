//! Base module used to define all entity descriptions that are used
//! to communicate with the server. This is highly dependant on the
//! version, so it's needed to update this on every client version.

use std::io::{self, Read};

pub mod interface;

pub mod account;


/// A trait to be implemented on all entities.
pub trait Entity {

    type Client;
    type Server;

    fn decode_client<R: Read>(idx: u16, read: R) -> io::Result<Self::Client>;

    fn decode_server<R: Read>(idx: u16, read: R) -> io::Result<Self::Server>;
    
}






/// The login entity. 
/// 
/// ID: 11
#[derive(Debug)]
pub struct Login {
    /// The database identifier of the account. It's the same identifier
    /// has the one publicly available through the Wargaming API. 
    /// 
    /// Such as '518858105' for player 'Mindstorm38_'.
    pub account_db_id: String,
}
