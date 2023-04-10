//! Base app interface.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::net::{SocketAddrV4, SocketAddr};
use std::sync::Arc;
use std::io;

use crypto_common::KeyInit;
use blowfish::Blowfish;
use rsa::RsaPrivateKey;

use crate::net::element::login::{
    Ping, 
    LoginRequest, LoginRequestEncryption, 
    LoginResponse, LoginResponseEncryption,
    ChallengeResponse, CuckooCycleResponse,
};

use crate::net::bundle::BundleElement;
use super::{Interface, InterfaceShared, InterfacePeer};


/// Interface implementation for login app.
pub struct LoginAppInterface {
    pub inner: Interface<LoginApp>,
}

impl LoginAppInterface {

    pub fn new(addr: SocketAddrV4) -> io::Result<Self> {

        let mut inner = Interface::new(addr, LoginApp {
            priv_key: None,
            clients: HashMap::new(),
        })?;

        inner.register(0x00, LoginApp::on_login_request, LoginApp::login_request_config);
        inner.register_simple(0x02, LoginApp::on_ping);
        inner.register_simple(0x03, LoginApp::on_challenge_response);

        Ok(Self { inner })

    }

}


/// Shared data for the login app.
pub struct LoginApp {
    /// Optional private key to set if encryption is enabled on the 
    /// login app.
    priv_key: Option<Arc<RsaPrivateKey>>,
    /// The mapping of clients to their socket address, it's used to
    /// track them during the login process.
    clients: HashMap<SocketAddr, LoginClient>,
}

impl LoginApp {

    /// Element handler for ping elements, this implementation send back
    /// the ping element has a reply.
    fn on_ping(&mut self, element: BundleElement<Ping>, mut peer: InterfacePeer<Self>) {
        peer.element_writer().write_simple_reply(element.element, element.request_id.unwrap());
    }

    /// Internal function to get access to a client.
    fn ensure_client(&mut self, addr: SocketAddr) -> &mut LoginClient {
        match self.clients.entry(addr) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(LoginClient::new(addr)),
        }
    }

    /// Callback to give the encryption used to decode a login request.
    fn login_request_config(&mut self, _addr: SocketAddr) -> LoginRequestEncryption {
        if let Some(priv_key) = &self.priv_key {
            LoginRequestEncryption::Server(priv_key.clone())
        } else {
            LoginRequestEncryption::Clear
        }
    }

    /// Element handler for login requests.
    fn on_login_request(&mut self, element: BundleElement<LoginRequest>, peer: InterfacePeer<Self>) {

        let client = self.ensure_client(peer.addr());

        let bf = client.blowfish.insert(Arc::new(Blowfish::new_from_slice(&element.element.blowfish_key).unwrap()));
        let encryption = LoginResponseEncryption::Encrypted(bf.clone());

    }

    fn on_challenge_response(&mut self, element: BundleElement<ChallengeResponse<CuckooCycleResponse>>, peer: InterfacePeer<Self>) {

    }

}

impl InterfaceShared for LoginApp {}


/// A structure for tracking an individual client tg
#[derive(Debug)]
pub struct LoginClient {
    addr: SocketAddr,
    blowfish: Option<Arc<Blowfish>>,
    challenge_complete: bool,
}

impl LoginClient {

    #[inline]
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            blowfish: None,
            challenge_complete: false,
        }
    }

}
