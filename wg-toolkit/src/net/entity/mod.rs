//! Base module used to define all entity descriptions that are used
//! to communicate with the server. This is highly dependant on the
//! version, so it's needed to update this on every client version.


/// The login entity. 
/// 
/// ID: 11
#[derive(Debug)]
pub struct Login {
    /// The database identifier of the account. It's the same identifier
    /// has the one publicly available through the wargaming API. 
    /// 
    /// Such as '518858105' for player 'Mindstorm38_'.
    pub account_db_id: String,
}

/// The account entity.
/// 
/// ID: 1
#[derive(Debug)]
pub struct Account {
    /// Part of the `AccountVersion.def` interface, just used by the 
    /// python app to check that game version is coherent.
    /// 
    /// For example `eu_1.19.1_4` as of this writing.
    pub required_version: String,
    /// The name of the account.
    pub name: String,
    
    pub initial_server_settings: String,
}
