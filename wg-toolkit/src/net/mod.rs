//! BigWorld/Core network protocol and applications definition.
//! 
//! The network protocol works with applications. The following application
//! are internally defined in the engine:
//! 
//! - Login app, it receives the initial connection request from the client
//!   with initial RSA encryption and transfers the client to the base app
//!   once it's successful.
//! 
//! - Client app, this is the client-side application.
//! 
//! - Base app, the application used for garage.
//! 
//! - Cell app, the application used for games.

pub mod packet;
pub mod element;
pub mod bundle;
pub mod filter;
pub mod cuckoo;

pub mod entity;

pub mod app;
