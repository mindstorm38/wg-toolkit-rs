//! Implementation of Fowler/Noll/Vo hash algorithm.

const FNV_32_PRIME: u64 = 0x01000193;
const FNV_64_PRIME: u64 = 0x100000001b3;

const FNV0_32_INIT: u64 = 0;
const FNV0_64_INIT: u64 = 0;
const FNV1_32_INIT: u64 = 0x811c9dc5;
const FNV1A_32_INIT: u64 = FNV1_32_INIT;
const FNV1_64_INIT: u64 = 0xcbf29ce484222325;
const FNV1A_64_INIT: u64 = FNV1_64_INIT;


/// Core FNV hash algorithm used in FNV0 and FNV1.
fn fnv(data: &[u8], hash_init: u64, fnv_prime: u64, fnv_size: u64) -> u64 {
    let mut hash = hash_init;
    for &byte in data {
        hash = (hash.wrapping_mul(fnv_prime) & fnv_size) ^ byte as u64;
    }
    hash
}

/// Alternative FNV hash algorithm used in FNV-1a.
fn fnva(data: &[u8], hash_init: u64, fnv_prime: u64, fnv_size: u64) -> u64 {
    let mut hash = hash_init;
    for &byte in data {
        hash = (hash ^ byte as u64).wrapping_mul(fnv_prime) & fnv_size;
    }
    hash
}

/// Returns the 32 bit FNV-0 hash value for the given data.
pub fn fnv0_32(data: &[u8]) -> u32 {
    fnv(data, FNV0_32_INIT, FNV_32_PRIME, u32::MAX as u64) as u32
}

/// Returns the 32 bit FNV-1 hash value for the given data.
pub fn fnv1_32(data: &[u8]) -> u32 {
    fnv(data, FNV1_32_INIT, FNV_32_PRIME, u32::MAX as u64) as u32
}

/// Returns the 32 bit FNV-1a hash value for the given data.
pub fn fnv1a_32(data: &[u8]) -> u32 {
    fnva(data, FNV1A_32_INIT, FNV_32_PRIME, u32::MAX as u64) as u32
}

/// Returns the 64 bit FNV-0 hash value for the given data.
pub fn fnv0_64(data: &[u8]) -> u64 {
    fnva(data, FNV0_64_INIT, FNV_64_PRIME, u64::MAX)
}

/// Returns the 64 bit FNV-0 hash value for the given data.
pub fn fnv1_64(data: &[u8]) -> u64 {
    fnva(data, FNV1_64_INIT, FNV_64_PRIME, u64::MAX)
}

/// Returns the 64 bit FNV-0 hash value for the given data.
pub fn fnv1a_64(data: &[u8]) -> u64 {
    fnva(data, FNV1A_64_INIT, FNV_64_PRIME, u64::MAX)
}
