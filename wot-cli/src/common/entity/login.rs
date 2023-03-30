use std::io::{self, Read, Write};

use wgtk::util::io::*;
use wgtk::net::element::SimpleElement;


/// The Login entity.
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

impl SimpleElement for Login {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_string_variable(&self.account_db_id)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self {
            account_db_id: read.read_string_variable()?,
        })
    }

}


/// Describe methods sent from the server to the client when the selected
/// entity is [`Login`].
pub enum LoginClientMethod {
    /// Kick the client with a periphery identifier.
    KickFromServer {
        checkout_periphery_id: i32,
    },
    /// Send the login queue number.
    QueueNumber {
        queue_number: u64,
    },
    /// Set the periphery rooting group for the client.
    PeripheryRoutingGroup {
        /// The routing group text identifier.
        routing_group: String,
        /// The list of available periphery identifiers available to the
        /// client. Note that theses groups and periphery identifiers are
        /// sent as part of the global server settings:
        /// [`crate::common::ServerSettings`].
        available_hosts: Vec<i32>,
    },
}



impl LoginClientMethod {

    // TODO: Note that not all methods have the same length (routing groups has a length prefix).

    /// Return the method index.
    pub fn index(&self) -> u8 {
        match self {
            LoginClientMethod::KickFromServer { .. } => 0,
            LoginClientMethod::QueueNumber { .. } => 1,
            LoginClientMethod::PeripheryRoutingGroup { .. } => 2,
        }
    }

}

impl SimpleElement for LoginClientMethod {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        match *self {
            LoginClientMethod::KickFromServer { checkout_periphery_id } => write.write_i32(checkout_periphery_id),
            LoginClientMethod::QueueNumber { queue_number } => write.write_u64(queue_number),
            LoginClientMethod::PeripheryRoutingGroup { 
                ref routing_group, 
                ref available_hosts 
            } => {
                write.write_string_variable(&routing_group)?;
                write.write_pickle(&available_hosts)
            }
        }
    }

    fn decode<R: Read>(read: R, len: usize) -> io::Result<Self> {
        todo!()
    }

}
