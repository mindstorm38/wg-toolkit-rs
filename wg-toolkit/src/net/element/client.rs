//! Definition of the elements that can be sent from server to client
//! once connected to the base application..

use glam::Vec3A;


#[derive(Debug)]
pub struct Authenticate {
    pub session_key: u32,
}

#[derive(Debug)]
pub struct BandwidthNotification {
    pub bitrate: u32,
}

#[derive(Debug)]
pub struct UpdateFrequencyNotification {
    pub frequency: u8,
}


/// An avatar update.
#[derive(Debug)]
pub struct AvatarUpdate {
    pub id: AvatarUpdateId,
    /// Position X, Y, Z.
    pub pos: Vec3A,
    /// Direction Yaw, Pitch, Roll.
    pub dir: Vec3A,
}

/// The entity ID for an avatar update.
#[derive(Debug)]
pub enum AvatarUpdateId {
    /// The entity ID is given directly without aliasing.
    NoAlias(u32),
    /// An alias for the entity ID, referring to an internal table of alias.
    Alias(u8),
}
