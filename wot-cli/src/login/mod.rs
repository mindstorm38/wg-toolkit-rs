use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::io;

use wgtk::util::TruncateFmt;

use wgtk::net::interface::{Shared, LoginInterface, LoginShared};


/// A wot-specific login app implementation.
pub struct WotLoginAppInterface {
    pub inner: LoginInterface<WotLoginApp>,
}

impl WotLoginAppInterface {

    pub fn new(addr: SocketAddrV4) -> io::Result<Self> {
        Ok(Self {
            inner: LoginInterface::new(addr, WotLoginApp {
                
            })?,
        })
    }

}


pub struct WotLoginApp {

}

impl Shared for WotLoginApp {}
impl LoginShared for WotLoginApp {}
