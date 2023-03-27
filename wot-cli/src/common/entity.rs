//! This module contains World of Tanks entities and codecs.

use std::io::{self, Read, Write};
use std::borrow::Cow;

use wgtk::util::io::*;
use wgtk::net::element::SimpleElement;

use super::server_settings::ServerSettings;


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
        write.write_string_variable(&self.account_db_id)?;
        write.write_u8(0) // I don't know why there is a terminating zero.
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self {
            account_db_id: read.read_string_variable()?,
        })
    }

}


/// The account entity. The lifetime is for the initial server settings.
/// 
/// ID: 1
#[derive(Debug)]
pub struct Account<'a> {
    /// Part of the `AccountVersion.def` interface, just used by the 
    /// python app to check that game version is coherent.
    /// 
    /// For example `eu_1.19.1_4` as of this writing.
    pub required_version: String,
    /// The name of the account.
    pub name: String,
    /// A shared pointer to server settings.
    pub initial_server_settings: Cow<'a, Box<ServerSettings>>,
}

impl SimpleElement for Account<'_> {
    
    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_string_variable(&self.required_version)?;
        write.write_string_variable(&self.name)?;
        write.write_pickle(&**self.initial_server_settings)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self {
            required_version: read.read_string_variable()?,
            name: read.read_string_variable()?,
            initial_server_settings: Cow::Owned(Box::new(read.read_pickle()?)),
        })
    }

}
