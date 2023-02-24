//! Cuckoo cycle challenge implementation.

use std::collections::HashSet;

use sha2::{Sha256, Digest};


pub const SHIFT_SIZE: usize = 25;
pub const PROOF_SIZE: usize = 42;
pub const MAX_PATH_LEN: usize = 8192;

pub const FULL_SIZE: usize = 1 << SHIFT_SIZE;
pub const HALF_SIZE: usize = FULL_SIZE / 2;
pub const NODE_MASK: usize = HALF_SIZE - 1;
pub const CUCKOO_SIZE: usize = FULL_SIZE + 1;


#[derive(Clone)]
pub struct SipHashContext {
    v0: u64,
    v1: u64,
    v2: u64,
    v3: u64,
}

impl SipHashContext {

    pub fn with_prefix(prefix: &[u8]) -> Self {
        
        let hash_raw = Sha256::new_with_prefix(prefix).finalize();
        let hash = hash_raw.as_slice();
        let k0 = u64::from_le_bytes(hash[0..8].try_into().unwrap());
        let k1 = u64::from_le_bytes(hash[8..16].try_into().unwrap());

        Self {
            v0: k0 ^ 0x736f6d6570736575,
            v1: k1 ^ 0x646f72616e646f6d,
            v2: k0 ^ 0x6c7967656e657261,
            v3: k1 ^ 0x7465646279746573,
        }

    }

    pub fn round(&mut self) {
        self.v0 += self.v1;
        self.v2 += self.v3;
        self.v1 = self.v1.rotate_left(13);
        self.v3 = self.v3.rotate_left(16);
        self.v1 ^= self.v0;
        self.v3 ^= self.v2;
        self.v0 = self.v0.rotate_left(32);
        self.v2 += self.v1;
        self.v0 += self.v3;
        self.v1 = self.v1.rotate_left(17);
        self.v3 = self.v3.rotate_left(21);
        self.v1 ^= self.v2;
        self.v3 ^= self.v0;
        self.v2 = self.v2.rotate_left(32);
    }

    pub fn sip_hash24(&self, nonce: u64) -> u64 {
        
        let mut sip = self.clone();

        sip.v3 ^= nonce;
        sip.round();
        sip.round();
        sip.v0 ^= nonce;
        sip.v2 ^= 0xFF;
        sip.round();
        sip.round();
        sip.round();
        sip.round();
        
        sip.v0 ^ sip.v1 ^ sip.v2 ^ sip.v3

    }

    pub fn sip_node(&self, nonce: u64, uorv: u32) -> usize {
        (self.sip_hash24(2 * nonce + uorv as u64) & NODE_MASK as u64) as usize
    }

    pub fn sip_edge(&self, nonce: u64) -> (usize, usize) {
        (
            self.sip_node(nonce, 0),
            self.sip_node(nonce, 1),
        )
    }

}


pub struct CuckooContext {
    sip_hash: SipHashContext,
    easiness: u64,
    cuckoo: Box<[usize; CUCKOO_SIZE]>
}

impl CuckooContext {

    pub fn new(easiness: u64, prefix: &[u8]) -> Self {
        Self {
            easiness,
            sip_hash: SipHashContext::with_prefix(prefix),
            cuckoo: Box::new([0; CUCKOO_SIZE])
        }
    }

    pub fn work(&mut self) -> Option<Vec<u64>> {

        let mut us = Box::new([0; MAX_PATH_LEN]);
        let mut vs = Box::new([0; MAX_PATH_LEN]);

        for nonce in 0..self.easiness {

            let (mut u0, mut v0) = self.sip_hash.sip_edge(nonce);
            u0 += 1;
            v0 += 1 + HALF_SIZE;

            let u = self.cuckoo[u0];
            let v = self.cuckoo[v0];

            if u == v0 || v == u0 {
                continue // ignore duplicate edges
            }

            us[0] = u0;
            vs[0] = v0;

            let mut nu = path(&self.cuckoo, u, &mut us).unwrap();
            let mut nv = path(&self.cuckoo, v, &mut vs).unwrap();

            if us[nu] == vs[nv] {

                let min = nu.min(nv);
                loop {
                    nu -= min;
                    nv -= min;
                    if us[nu] != vs[nv] {
                        break;
                    }
                    nu += 1;
                    nv += 1;
                }

                let len = nu + nv + 1;

                if len == PROOF_SIZE {
                    return Some(self.solution(&us, nu, &vs, nv));
                }

            } else if nu < nv {
                while nu != 0 {
                    nu -= 1;
                    self.cuckoo[us[nu + 1]] = us[nu];
                }
                self.cuckoo[u0] = v0;
            } else {
                while nv != 0 {
                    nv -= 1;
                    self.cuckoo[vs[nv + 1]] = vs[nv];
                }
                self.cuckoo[v0] = u0;
            }

        }

        None

    }

    pub fn solution(&self, us: &[usize; MAX_PATH_LEN], nu: usize, vs: &[usize; MAX_PATH_LEN], nv: usize) -> Vec<u64> {

        let mut cycle = HashSet::new();
        let mut solution = Vec::new();

        cycle.insert((us[0], vs[0]));
        
        for nu in (0..nu).rev() {
            cycle.insert((us[(nu + 1) & !1], us[nu | 1]));
        }

        for nv in (0..nv).rev() {
            cycle.insert((vs[nv | 1], vs[(nv + 1) & !1]));
        }

        for nonce in 0..self.easiness {
            
            let mut edge = self.sip_hash.sip_edge(nonce);
            edge.0 += 1;
            edge.1 += 1;

            if cycle.remove(&edge) {
                solution.push(nonce);
            }

        }

        solution

    }

}


fn path(cuckoo: &[usize; CUCKOO_SIZE], mut u: usize, us: &mut [usize; MAX_PATH_LEN]) -> Option<usize> {

    let mut nu = 0isize;

    while u != 0 {

        u = cuckoo[u];
        nu += 1;

        if nu >= MAX_PATH_LEN as _ {

            loop {
                nu -= 1;
                if nu == 0 && us[nu as usize] != u {
                    break;
                }
            }

            if nu < 0 {
                return None;
            }

        }

        us[nu as usize] = u;

    }

    Some(nu as usize)

}
