use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::io;

use wgtk::util::TruncateFmt;

use wgtk::net::interface::{Shared, LoginAppInterface, LoginAppShared};


/// A wot-specific login app implementation.
pub struct WotLoginAppInterface {
    pub inner: LoginAppInterface<WotLoginApp>,
}

impl WotLoginAppInterface {

    pub fn new(addr: SocketAddrV4) -> io::Result<Self> {
        Ok(Self {
            inner: LoginAppInterface::new(addr, WotLoginApp {
                
            })?,
        })
    }

}


pub struct WotLoginApp {

}

impl Shared for WotLoginApp {}
impl LoginAppShared for WotLoginApp {}
