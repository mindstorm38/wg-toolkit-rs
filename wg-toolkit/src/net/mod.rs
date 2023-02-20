//! BigWorld/Core network protocol and applications definition.

pub mod packet;
pub mod element;
pub mod bundle;
pub mod proxy;
pub mod filter;
pub mod cuckoo;

pub mod app;


// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ReliableType {
//     None = 0,
//     Driver = 1,
//     Passenger = 2,
//     Critical = 3,
// }
