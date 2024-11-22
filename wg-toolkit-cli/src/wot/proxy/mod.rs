//! Proxy login and base app used for debugging exchanged messages.

use std::net::{SocketAddr, SocketAddrV4};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::{fmt, io, thread};
use std::fs::File;

use tracing::{error, info, instrument, warn};

use flate2::read::ZlibDecoder;
use blowfish::Blowfish;
use rsa::{RsaPrivateKey, RsaPublicKey};

use wgtk::net::element::{DebugElementUndefined, DebugElementVariable16, SimpleElement};
use wgtk::net::bundle::{Bundle, ElementReader, TopElementReader};

use wgtk::net::app::{login, base, client, proxy};
use wgtk::net::app::common::entity::Entity;
use wgtk::net::app::proxy::PacketDirection;

use wgtk::util::io::serde_pickle_de_options;
use wgtk::util::TruncateFmt;

use crate::CliResult;
use super::gen;


pub fn run(
    login_app_addr: SocketAddrV4,
    real_login_app_addr: SocketAddrV4,
    base_app_addr: SocketAddrV4,
    encryption_key: Option<Arc<RsaPrivateKey>>,
    real_encryption_key: Option<Arc<RsaPublicKey>>,
) -> CliResult<()> {

    let mut login_app = login::proxy::App::new(login_app_addr.into(), real_login_app_addr.into(), real_encryption_key)
        .map_err(|e| format!("Failed to bind login app: {e}"))?;
    
    if let Some(encryption_key) = encryption_key {
        login_app.set_encryption(encryption_key);
    }

    login_app.set_forced_base_app_addr(base_app_addr);

    let base_app = proxy::App::new(base_app_addr.into())
        .map_err(|e| format!("Failed to bind base app: {e}"))?;

    let shared = Arc::new(ProxyShared {
        pending_clients: Mutex::new(HashMap::new()),
    });

    let login_thread = LoginProxyThread {
        app: login_app,
        shared: Arc::clone(&shared),
    };

    let base_thread = BaseProxyThread {
        app: base_app,
        shared,
        next_tick: None,
        entities: HashMap::new(),
        selected_entity_id: None,
        player_entity_id: None,
        partial_resources: HashMap::new(),
    };
    
    thread::scope(move |scope| {
        scope.spawn(move || login_thread.run());
        scope.spawn(move || base_thread.run());
    });

    Ok(())

}


#[derive(Debug)]
struct LoginProxyThread {
    app: login::proxy::App,
    shared: Arc<ProxyShared>,
}

#[derive(Debug)]
struct BaseProxyThread {
    app: proxy::App,
    shared: Arc<ProxyShared>,
    next_tick: Option<u8>,
    entities: HashMap<u32, &'static ProxyEntityType>,
    selected_entity_id: Option<u32>,
    player_entity_id: Option<u32>,
    partial_resources: HashMap<u16, ProxyPartialResource>,
}

#[derive(Debug)]
struct ProxyShared {
    pending_clients: Mutex<HashMap<SocketAddr, ProxyPendingClient>>,
}

#[derive(Debug)]
struct ProxyPendingClient {
    base_app_addr: SocketAddrV4,
    blowfish: Arc<Blowfish>,
}

impl LoginProxyThread {

    #[instrument(name = "login", skip_all)]
    fn run(mut self) {

        use login::proxy::Event;

        info!("Running on: {}", self.app.addr().unwrap());
        
        if self.app.has_encryption() {
            info!("Encryption enabled");
        }

        loop {
            match self.app.poll() {
                Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(%addr, "Error: {}", error.error);
                    } else {
                        warn!("Error: {}", error.error);
                    }
                }
                Event::Ping(ping) => {
                    info!(addr = %ping.addr, "Ping-Pong: {:?}", ping.latency);
                }
                Event::LoginSuccess(success) => {
                    info!(addr = %success.addr, "Login success");
                    self.shared.pending_clients.lock().unwrap().insert(success.addr, ProxyPendingClient { 
                        base_app_addr: success.real_base_app_addr,
                        blowfish: success.blowfish, 
                    });
                }
                Event::LoginError(error) => {
                    info!(addr = %error.addr, "Login error: {:?}", error.error);
                }
            }
        }

    }

}

impl BaseProxyThread {

    #[instrument(name = "base", skip_all)]
    fn run(mut self) {

        use proxy::Event;

        info!("Running on: {}", self.app.addr().unwrap());

        loop {
            match self.app.poll() {
                Event::IoError(error) => {
                    if let Some(addr) = error.addr {
                        warn!(%addr, "Error: {}", error.error);
                    } else {
                        warn!("Error: {}", error.error);
                    }
                }
                Event::Rejection(rejection) => {
                    if let Some(pending_client) = self.shared.pending_clients.lock().unwrap().remove(&rejection.addr) {
                        
                        info!("Rejection of known peer: {} (to {})", rejection.addr, pending_client.base_app_addr);
                        
                        self.app.bind_peer(
                            rejection.addr, 
                            SocketAddr::V4(pending_client.base_app_addr), 
                            Some(pending_client.blowfish),
                            None).unwrap();

                    } else {
                        warn!("Rejection of unknown peer: {}", rejection.addr);
                    }
                }
                Event::Bundle(bundle) => {
                    
                    let res = match bundle.direction {
                        PacketDirection::Out => self.read_out_bundle(bundle.addr, bundle.bundle),
                        PacketDirection::In => self.read_in_bundle(bundle.addr, bundle.bundle),
                    };

                    if let Err(e) = res {
                        error!(addr = %bundle.addr, "Error while reading bundle: ({:?}) {e}", bundle.direction);
                    }

                }
                    
            }
        }

    }

    fn read_out_bundle(&mut self, addr: SocketAddr, bundle: Bundle) -> io::Result<()> {

        let mut reader = bundle.element_reader();
        while let Some(elt) = reader.next_element() {
            match elt {
                ElementReader::Top(elt) => {
                    if !self.read_out_element(addr, elt)? {
                        break;
                    }
                }
                ElementReader::Reply(elt) => {
                    let request_id = elt.request_id();
                    let _elt = elt.read_simple::<()>()?;
                    warn!(%addr, "-> Reply element #{request_id}");
                    break;
                }
            }
        }

        Ok(())

    }

    fn read_out_element(&mut self, addr: SocketAddr, elt: TopElementReader) -> io::Result<bool> {
        
        use base::element::*;

        match elt.id() {
            ClientSessionKey::ID => {
                let elt = elt.read_simple::<ClientSessionKey>()?;
                info!(%addr, "-> Session key: 0x{:08X}", elt.element.session_key);
            }
            id if id::BASE_ENTITY_METHOD.contains(id) => {

                // Account::doCmdInt3 (AccountCommands.CMD_SYNC_DATA), exposed id: 0x0E, message id: 0x95

                if let Some(entity_id) = self.player_entity_id {
                    // Unwrap because selected entity should exist!
                    let entity_type = *self.entities.get(&entity_id).unwrap();
                    return (entity_type.base_entity_method)(addr, entity_id, elt);
                }

                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                warn!(%addr, "-> Base entity method (unknown selected entity): msg#{} {:?} (request: {:?})", id - id::BASE_ENTITY_METHOD.first, elt.element, elt.request_id);
                return Ok(false);

            }
            id => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                error!(%addr, "-> Top element #{id} {:?} (request: {:?})", elt.element, elt.request_id);
                return Ok(false);
            }
        }

        Ok(true)

    }

    fn read_in_bundle(&mut self, addr: SocketAddr, bundle: Bundle) -> io::Result<()> {

        let mut reader = bundle.element_reader();
        while let Some(elt) = reader.next_element() {
            match elt {
                ElementReader::Top(elt) => {
                    if !self.read_in_element(addr, elt)? {
                        break;
                    }
                }
                ElementReader::Reply(elt) => {
                    let request_id = elt.request_id();
                    let _elt = elt.read_simple::<()>()?;
                    warn!(%addr, "<- Reply element #{request_id}");
                    break;
                }
            }
        }

        Ok(())

    }

    fn read_in_element(&mut self, addr: SocketAddr, mut elt: TopElementReader) -> io::Result<bool> {

        use client::element::*;

        match elt.id() {
            UpdateFrequencyNotification::ID => {
                let ufn = elt.read_simple::<UpdateFrequencyNotification>()?;
                info!(%addr, "<- Update frequency: {} Hz, game time: {}", ufn.element.frequency, ufn.element.game_time);
            }
            TickSync::ID => {
                let ts = elt.read_simple::<TickSync>()?;
                if let Some(next_tick) = self.next_tick {
                    if next_tick != ts.element.tick {
                        warn!(%addr, "<- Tick missed, expected {next_tick}, got {}", ts.element.tick);
                    }
                }
                self.next_tick = Some(ts.element.tick.wrapping_add(1));
            }
            ResetEntities::ID => {

                let re = elt.read_simple::<ResetEntities>()?;

                info!(%addr, "<- Reset entities, keep player on base: {}, entities: {}", 
                    re.element.keep_player_on_base, self.entities.len());

                // Don't delete player entity if requested...
                let mut player_entity = None;
                if re.element.keep_player_on_base {
                    if let Some(player_entity_id) = self.player_entity_id {
                        player_entity = Some(self.entities.remove_entry(&player_entity_id).unwrap());
                    }
                }
                
                self.entities.clear();
                self.player_entity_id = None;
                
                // Restore player entity!
                if let Some((player_entity_id, player_entity)) = player_entity {
                    self.entities.insert(player_entity_id, player_entity);
                    self.player_entity_id = Some(player_entity_id);
                }

            }
            LoggedOff::ID => {
                let lo = elt.read_simple::<LoggedOff>()?;
                info!(%addr, "<- Logged off: {lo:?}");
            }
            CreateBasePlayerHeader::ID => {

                let cbp = elt.read_simple_stable::<CreateBasePlayerHeader>()?;

                if let Some(entity_type) = cbp.element.entity_type_id.checked_sub(1).and_then(|i| ENTITY_TYPES.get(i as usize)) {
                    self.entities.insert(cbp.element.entity_id, entity_type);
                    self.player_entity_id = Some(cbp.element.entity_id);
                    return (entity_type.create_base_player)(addr, elt);
                }

                self.player_entity_id = None;
                // It's possible to skip it because its len is variable.
                let dbg = elt.read_simple::<DebugElementVariable16<0>>()?;
                warn!(%addr, "<- Create base player with invalid entity type id: 0x{:02X}, {:?}", 
                    cbp.element.entity_type_id, dbg.element);

            }
            CreateCellPlayer::ID => {
                let ccp = elt.read_simple::<CreateCellPlayer>()?;
                warn!(%addr, "<- Create cell player: {:?}", ccp.element);
            }
            SelectPlayerEntity::ID => {
                let _spe = elt.read_simple::<SelectPlayerEntity>()?;
                if let Some(player_entity_id) = self.player_entity_id {
                    info!(%addr, "<- Select player entity: {player_entity_id}");
                } else {
                    warn!(%addr, "<- Select player entity: no player entity")
                }
                self.selected_entity_id = self.player_entity_id;
            }
            ResourceHeader::ID => {

                let rh = elt.read_simple::<ResourceHeader>()?;
                info!(%addr, "<- Resource header: {}", rh.element.id);

                // Intentionally overwrite any previous downloading resource!
                self.partial_resources.insert(rh.element.id, ProxyPartialResource {
                    description: rh.element.description,
                    sequence_num: 0,
                    data: Vec::new(),
                });

            }
            ResourceFragment::ID => {

                let rf = elt.read_simple::<ResourceFragment>()?;
                let res_id = rf.element.id;

                let Some(partial_resource) = self.partial_resources.get_mut(&res_id) else {
                    warn!(%addr, "<- Resource fragment: {res_id}, len: {}, missing header", rf.element.data.len());
                    return Ok(true);
                };

                if rf.element.sequence_num != partial_resource.sequence_num {
                    // Just forgetting about the resource!
                    warn!(%addr, "<- Resource fragment: {res_id}, len: {}, invalid sequence number, expected {}, got {}", 
                    rf.element.data.len(), partial_resource.sequence_num, rf.element.sequence_num);
                    let _ = self.partial_resources.remove(&res_id);
                    return Ok(true);
                }

                partial_resource.sequence_num += 1;
                partial_resource.data.extend_from_slice(&rf.element.data);
                info!(%addr, "<- Resource fragment: {res_id}, len: {}, sequence number: {}", 
                    rf.element.data.len(), partial_resource.sequence_num);
                
                // Process the finished fragment!
                if rf.element.last {

                    let resource = self.partial_resources.remove(&rf.element.id).unwrap();
                    
                    // See: scripts/client/game.py#L223
                    let (total_len, crc32) = match serde_pickle::value_from_reader(&resource.description[..], serde_pickle_de_options()) {
                        Ok(serde_pickle::Value::Tuple(values)) if values.len() == 2 => {
                            if let &[serde_pickle::Value::I64(total_len), serde_pickle::Value::I64(crc32)] = &values[..] {
                                (total_len as u32, crc32 as u32)
                            } else {
                                warn!(%addr, "<- Invalid resource description: unexpected values: {values:?}");
                                return Ok(true);
                            }
                        }
                        Ok(v) => {
                            warn!(%addr, "<- Invalid resource description: python: {v}");
                            return Ok(true);
                        }
                        Err(e) => {
                            warn!(%addr, "<- Invalid resource description: {e}");
                            return Ok(true);
                        }
                    };

                    let actual_total_len = resource.data.len();
                    if actual_total_len != total_len as usize {
                        warn!(%addr, "<- Invalid resource length, expected: {total_len}, got: {actual_total_len}");
                        return Ok(true);
                    }

                    let actual_crc32 = crc32fast::hash(&resource.data);
                    if actual_crc32 != crc32 {
                        warn!(%addr, "<- Invalid resource crc32, expected: 0x{crc32:08X}, got: 0x{actual_crc32:08X}");
                        return Ok(true);
                    }

                    info!(%addr, "<- Resource completed: {res_id}, len: {actual_total_len}, crc32: 0x{crc32:08X}");

                    // TODO: The full data looks like to be a zlib-compressed pickle.
                    // TODO: onCmdResponse for requested SYNC use RES_SUCCESS=0, RES_STREAM=1, RES_CACHE=2 for result_id
                    //       When RES_STREAM is used, then a resource (header+fragment) is expected with the associated request_id.

                    let zlib = ZlibDecoder::new(&resource.data[..]);
                    match serde_pickle::value_from_reader(zlib, serde_pickle_de_options()) {
                        Ok(val) => {
                            info!(%addr, "<- Resource: {}", TruncateFmt(&val, 3000));
                        }
                        Err(e) => {

                            warn!(%addr, "<- Resource: python error: {e}");

                            // FIXME: It appears that the current serde-pickle impl doesn't
                            // support recursive structures, however the structure that is 
                            // initially requested with 'CMD_SYNC_DATA' contains some.
                            // FIXME: The resource that is received by the from the chat
                            // command contains a "deque" object, which cannot be parsed
                            // so we get a "unresolved global reference" error.

                            let raw_file = PathBuf::from(format!("res_{crc32:08x}.pickle"));
                            info!(%addr, "<- Saving resource to: {}", raw_file.display());

                            let mut raw_writer = File::create(raw_file).unwrap();
                            std::io::copy(&mut ZlibDecoder::new(&resource.data[..]), &mut raw_writer).unwrap();

                        }
                    }

                }

            }
            id if id::ENTITY_METHOD.contains(id) => {

                // Account::msg#37 = onClanInfoReceived
                // Account::msg#39 = showGUI

                if let Some(entity_id) = self.selected_entity_id {
                    // Unwrap because selected entity should exist!
                    let entity_type = *self.entities.get(&entity_id).unwrap();
                    return (entity_type.entity_method)(addr, entity_id, elt);
                }

                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                warn!(%addr, "<- Entity method (unknown selected entity): msg#{} {:?} (request: {:?})", id - id::ENTITY_METHOD.first, elt.element, elt.request_id);
                return Ok(false);

            }
            id if id::ENTITY_PROPERTY.contains(id) => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                warn!(%addr, "<- Entity property: msg#{} {:?} (request: {:?})", id - id::ENTITY_PROPERTY.first, elt.element, elt.request_id);
                return Ok(false);
            }
            id => {
                let elt = elt.read_simple::<DebugElementUndefined<0>>()?;
                error!(%addr, "<- Top element #{id} {:?} (request: {:?})", elt.element, elt.request_id);
                return Ok(false);
            }
        }

        Ok(true)

    }

}

/// Describe a partial resource being download, a header must have been sent.
#[derive(Debug)]
struct ProxyPartialResource {
    /// The byte description sent in the resource header.
    description: Vec<u8>,
    /// The next sequence number expected, any other sequence number abort the download
    /// with an error.
    sequence_num: u8,
    /// The full assembled data.
    data: Vec<u8>,
}

/// Represent an entity type and its associated static functions.
#[derive(Debug)]
struct ProxyEntityType {
    create_base_player: fn(SocketAddr, TopElementReader) -> io::Result<bool>,
    entity_method: fn(SocketAddr, u32, TopElementReader) -> io::Result<bool>,
    base_entity_method: fn(SocketAddr, u32, TopElementReader) -> io::Result<bool>,
}

impl ProxyEntityType {

    const fn new<E>() -> Self
    where
        E: Entity + fmt::Debug,
        E::ClientMethod: fmt::Debug,
        E::BaseMethod: fmt::Debug,
    {
        Self {
            create_base_player: |addr, elt| {
                use client::element::CreateBasePlayer;
                let cbp = elt.read_simple::<CreateBasePlayer<E>>()?;
                info!(%addr, "<- Create base player: ({}) {:?}", cbp.element.entity_id, cbp.element.entity_data);
                Ok(true)
            },
            entity_method: |addr, entity_id, elt| {
                use client::element::EntityMethod;
                let em = elt.read_simple::<EntityMethod<E::ClientMethod>>()?;
                info!(%addr, "<- Entity method: ({entity_id}) {:?}", em.element.inner);
                Ok(true)
            },
            base_entity_method: |addr, entity_id, elt| {
                use base::element::BaseEntityMethod;
                let em = elt.read_simple::<BaseEntityMethod<E::BaseMethod>>()?;
                info!(%addr, "-> Base entity method: ({entity_id}) {:?}", em.element.inner);
                Ok(true)
            },
        }
    }

}

const ENTITY_TYPES: &[ProxyEntityType] = &[
    ProxyEntityType::new::<gen::entity::Account>(),
    ProxyEntityType::new::<gen::entity::Avatar>(),
    ProxyEntityType::new::<gen::entity::ArenaInfo>(),
    ProxyEntityType::new::<gen::entity::ClientSelectableObject>(),
    ProxyEntityType::new::<gen::entity::HangarVehicle>(),
    ProxyEntityType::new::<gen::entity::Vehicle>(),
    ProxyEntityType::new::<gen::entity::AreaDestructibles>(),
    ProxyEntityType::new::<gen::entity::OfflineEntity>(),
    ProxyEntityType::new::<gen::entity::Flock>(),
    ProxyEntityType::new::<gen::entity::FlockExotic>(),
    ProxyEntityType::new::<gen::entity::Login>(),
];
