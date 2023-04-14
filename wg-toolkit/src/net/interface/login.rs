//! Login app interface.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::net::{SocketAddrV4, SocketAddr};
use std::sync::Arc;
use std::io;

use crypto_common::KeyInit;
use blowfish::Blowfish;
use rand::RngCore;
use rsa::RsaPrivateKey;
use rand::rngs::OsRng;

use crate::net::bundle::BundleElement;
use crate::net::element::login::{
    id as login_id,
    Ping, 
    LoginRequest, LoginRequestEncryption, 
    LoginResponse, LoginResponseEncryption,
    ChallengeResponse, CuckooCycleResponse, 
    LoginChallenge, LoginSuccess, LoginError, 
};

use super::{Interface, Shared, Peer};


/// Interface implementation for login app.
pub struct LoginAppInterface<S: LoginAppShared> {
    pub inner: Interface<LoginApp<S>>,
}

impl<S: LoginAppShared> LoginAppInterface<S> {

    pub fn new(addr: SocketAddrV4, shared: S) -> io::Result<Self> {

        let mut inner = Interface::new(addr, LoginApp {
            shared,
            priv_key: None,
            clients: HashMap::new(),
        })?;

        inner.register(login_id::LOGIN_REQUEST, LoginApp::on_login_request, LoginApp::login_request_config);
        inner.register_simple(login_id::PING, LoginApp::on_ping);
        inner.register_simple(login_id::CHALLENGE_RESPONSE, LoginApp::on_challenge_response);

        Ok(Self { inner })

    }

}

/// Shared data for the login app.
pub struct LoginApp<S: LoginAppShared> {
    /// Inner login-app-specific shared data.
    #[allow(unused)] // TODO: remove
    shared: S,
    /// Optional private key to set if encryption is enabled on the 
    /// login app.
    priv_key: Option<Arc<RsaPrivateKey>>,
    /// The mapping of clients to their socket address, it's used to
    /// track them during the login process.
    clients: HashMap<SocketAddr, Client>,
}

impl<S: LoginAppShared> LoginApp<S> {

    /// Element handler for ping elements, this implementation send back
    /// the ping element has a reply.
    fn on_ping(&mut self, element: BundleElement<Ping>, mut peer: Peer<Self>) {
        peer.element_writer().write_simple_reply(element.element, element.request_id.unwrap());
    }

    /// Internal function to get access to a client.
    fn ensure_client(&mut self, addr: SocketAddr) -> &mut Client {
        match self.clients.entry(addr) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Client::new()),
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
    fn on_login_request(&mut self, element: BundleElement<LoginRequest>, mut peer: Peer<Self>) {

        // FIXME: We are currently not checking anything prior to connection.
        // No password, no challenge is required.

        let request_id = element.request_id.expect("login request must have a request id");

        // Get the client (create it if not available) associated to the address.
        let client = self.ensure_client(peer.addr());
        // Initialize the client's blowfish encryption.
        let bf = Arc::new(Blowfish::new_from_slice(&element.element.blowfish_key).unwrap());
        client.blowfish = Some(bf.clone());

        // Encryption config used for replies' encoding.
        let encryption = LoginResponseEncryption::Encrypted(bf);

        if !client.challenge_complete {
            
            let cuckoo_prefix_value = OsRng.next_u64();
            let cuckoo_prefix = format!("{cuckoo_prefix_value:>02X}");
            let cuckoo_easiness = 0.9;

            let challenge = LoginChallenge::CuckooCycle { 
                prefix: cuckoo_prefix, 
                max_nonce: ((1 << 20) as f32 * cuckoo_easiness) as _,
            };

            peer.element_writer().write_reply(LoginResponse::Challenge(challenge), &encryption, request_id);

        } else {

            let res;
            match self.shared.try_login(&element.element) {
                Ok((addr, login_key)) => {
                    res = LoginResponse::Success(LoginSuccess {
                        addr,
                        login_key,
                        server_message: String::new(),
                    });
                }
                Err(()) => {
                    res = LoginResponse::Error(LoginError::InvalidPassword, String::new());
                }
            }

            peer.element_writer().write_reply(res, &encryption, request_id);

        }

    }

    /// Element handler for challenge responses.
    fn on_challenge_response(&mut self, _element: BundleElement<ChallengeResponse<CuckooCycleResponse>>, peer: Peer<Self>) {
        
        // FIXME: For now we don't check if the challenge has been actually completed.
        self.ensure_client(peer.addr()).challenge_complete = true;

    }

}

impl<S: LoginAppShared> Shared for LoginApp<S> { }


/// Login-app-specific shared interface, the implementor must also
/// implement [`InterfaceShared`] as it will used as a callback for
/// global events.
pub trait LoginAppShared: Shared {

    /// Try to login, if successful it must return a tuple containing
    /// the base app address and login key to use. If it's a success,
    /// this function is responsible of registering this login key
    /// against a database.
    /// 
    /// If there is an error, it will be always returned that it's a 
    /// password problem.
    fn try_login(&mut self, request: &LoginRequest) -> Result<(SocketAddrV4, u32), ()>;

}


/// A structure for tracking an individual client tg
#[derive(Debug)]
struct Client {
    blowfish: Option<Arc<Blowfish>>,
    challenge_complete: bool,
}

impl Client {

    #[inline]
    pub fn new() -> Self {
        Self {
            blowfish: None,
            challenge_complete: false,
        }
    }

}
