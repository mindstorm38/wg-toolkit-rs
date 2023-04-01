use std::io::{self, Read, Write};
use std::borrow::Cow;

use wgtk::util::io::*;
use wgtk::net::element::SimpleElement;

use crate::common::server_settings::ServerSettings;


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
    
    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_string_variable(&self.required_version)?;
        write.write_string_variable(&self.name)?;
        write.write_pickle(&**self.initial_server_settings)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self {
            required_version: read.read_string_variable()?,
            name: read.read_string_variable()?,
            initial_server_settings: Cow::Owned(Box::new(read.read_pickle()?)),
        })
    }

}
