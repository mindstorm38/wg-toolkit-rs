//! Base module used to define all entity descriptions that are used
//! to communicate with the server. This is highly dependant on the
//! version, so it's needed to update this on every client version.

use std::io::{self, Read};

pub mod interface;

pub mod account;
