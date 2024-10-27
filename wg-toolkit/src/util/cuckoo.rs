//! Cuckoo cycle challenge implementation.
//! 
//! Ref: https://eprint.iacr.org/2014/059.pdf
//! Ref (BigWorld): programming\bigworld\lib\connection\cuckoo_cycle_login_challenge_factory.cpp

use std::collections::HashSet;
use std::num::Wrapping;

use sha2::{Sha256, Digest};


const BW_SIZE_SHIFT: u32 = 20;
const BW_MAX_PATH_LEN: usize = 8192;
const BW_PROOF_SIZE: usize = 42;

/// Cuckoo context with support for BigWorld changes.
#[derive(Debug)]
pub struct CuckooContext {
    sip_hash: SipHashContext,
    max_nonce: u32,
}

impl CuckooContext {

    pub fn new(max_nonce: u32, prefix: &[u8]) -> Self {
        Self {
            max_nonce,
            sip_hash: SipHashContext::new_with_prefix(prefix),
        }
    }

    #[inline]
    fn sip_node(&self, size: u32, nonce: u32, uorv: u32) -> u32 {
        let nonce = (Wrapping(nonce) * Wrapping(2) + Wrapping(uorv)).0;
        let hash = self.sip_hash.sip_hash24(nonce as u64) as u32;
        hash & (size / 2 - 1) // Calculate node_mask from half_size from size
    }

    #[inline]
    fn sip_edge(&self, size: u32, nonce: u32) -> (u32, u32) {
        (
            self.sip_node(size, nonce, 0),
            self.sip_node(size, nonce, 1),
        )
    }

    #[inline]
    fn sip_edge_offset(&self, size: u32, nonce: u32) -> (u32, u32) {
        let (u0, v0) = self.sip_edge(size, nonce);
        (u0 + 1, v0 + 1 + (size / 2))
    }

    pub fn work_bw(&self) -> Option<Vec<u32>> {
        self.work(BW_SIZE_SHIFT, BW_MAX_PATH_LEN, BW_PROOF_SIZE)
    }

    pub fn verify_bw(&self, solution: &[u32]) -> bool {
        solution.len() == BW_PROOF_SIZE && self.verify(BW_SIZE_SHIFT, solution)
    }

    pub fn work(&self, 
        size_shift: u32,
        max_path_len: usize,
        proof_size: usize,
    ) -> Option<Vec<u32>> {

        assert!(size_shift < 32);
        assert!(max_path_len > 0);
        assert!(proof_size > 0);

        let size = 1u32 << size_shift;

        let mut cuckoo = vec![0u32; size as usize + 1];
        let mut us = vec![0u32; max_path_len];
        let mut vs =  vec![0u32; max_path_len];

        for nonce in 0..self.max_nonce {

            let (u0, v0) = self.sip_edge_offset(size, nonce);

            let u = cuckoo[u0 as usize];
            let v = cuckoo[v0 as usize];

            if u == v0 || v == u0 {
                continue // ignore duplicate edges
            }

            us[0] = u0;
            vs[0] = v0;

            // The '?' is BigWorld-specific because original impl just exit process 
            // without error.
            let mut nu = self.path(&cuckoo, u, &mut us)?;
            let mut nv = self.path(&cuckoo, v, &mut vs)?;

            if us[nu as usize] == vs[nv as usize] {

                let min = nu.min(nv);
                nu -= min;
                nv -= min;
                while us[nu as usize] != vs[nv as usize] {
                    nu += 1;
                    nv += 1;
                }

                let len = nu as usize + nv as usize + 1;
                if len == proof_size {
                    return Some(self.solution(size, &us, nu, &vs, nv));
                }

            } else if nu < nv {
                while nu > 0 {
                    nu -= 1;
                    cuckoo[us[nu as usize + 1] as usize] = us[nu as usize];
                }
                cuckoo[u0 as usize] = v0;
            } else {
                while nv > 0 {
                    nv -= 1;
                    cuckoo[vs[nv as usize + 1] as usize] = vs[nv as usize];
                }
                cuckoo[v0 as usize] = u0;
            }

        }

        None

    }

    fn path(&self, cuckoo: &[u32], mut u: u32, us: &mut [u32]) -> Option<u32> {
        let mut nu = 0u32;
        while u != 0 {
            nu += 1;
            if nu as usize >= us.len() {  // MAX_PATH_LEN
                // while (nu-- && us[nu] != u); if (nu < 0) return nu;
                // The origin impl use '< 0' values as a 'none', so we use checked sub
                // so that any value below 0 like original impl return none.
                loop {
                    nu = nu.checked_sub(1)?;
                    if us[nu as usize] == u { break; }
                }
            }
            us[nu as usize] = u;
            u = cuckoo[u as usize];
        }
        Some(nu)
    }

    fn solution(&self, 
        size: u32, 
        us: &[u32], nu: u32, 
        vs: &[u32], nv: u32,
    ) -> Vec<u32> {

        debug_assert_eq!(us.len(), vs.len(), "max path len not equal");

        let mut cycle = HashSet::new();
        let mut solution = Vec::new();

        cycle.insert((us[0], vs[0]));
        
        for nu in (0..nu).rev() {
            cycle.insert((us[((nu + 1) & !1) as usize], us[(nu | 1) as usize]));
        }

        for nv in (0..nv).rev() {
            cycle.insert((vs[(nv | 1) as usize], vs[((nv + 1) & !1) as usize]));
        }

        for nonce in 0..self.max_nonce {
            let edge = self.sip_edge_offset(size, nonce);
            if cycle.remove(&edge) {
                solution.push(nonce);
            }
        }

        solution

    }

    pub fn verify(&self, 
        size_shift: u32,
        solution: &[u32]
    ) -> bool {

        assert!(size_shift < 32);

        if solution.is_empty() {
            return false;
        }

        let size = 1u32 << size_shift;
        let proof_size = solution.len();

        let mut us = vec![0u32; proof_size];
        let mut vs = vec![0u32; proof_size];

        for k in 0..proof_size {
            
            if solution[k] >= self.max_nonce || (k > 0 && solution[k] <= solution[k - 1]) {
                return false;
            }

            let (u0, v0) = self.sip_edge(size, solution[k]);
            us[k] = u0;
            vs[k] = v0;

        }

        let mut i = 0usize;
        let mut n = proof_size;

        loop {

            let mut j = i;
            
            for k in 0..proof_size {
                if k != i && vs[k] == vs[i] {
                    if j != i {
                        return false;
                    }
                    j = k;
                }
            }

            if j == i {
                return false;
            }

            i = j;

            for k in 0..proof_size {
                if k != j && us[k] == us[j] {
                    if i != j {
                        return false;
                    }
                    i = k;
                }
            }

            if i == j {
                return false;
            }

            n -= 2;

            if i == 0 {
                break
            }

        }

        n == 0

    }

}

/// Internal state for sip hash context.
#[derive(Debug, Clone)]
struct SipHashState {
    v0: Wrapping<u64>,
    v1: Wrapping<u64>,
    v2: Wrapping<u64>,
    v3: Wrapping<u64>,
}

impl SipHashState {

    fn round(&mut self) {
        self.v0 += self.v1;
        self.v2 += self.v3;
        self.v1 = Wrapping(self.v1.0.rotate_left(13));
        self.v3 = Wrapping(self.v3.0.rotate_left(16));
        self.v1 ^= self.v0;
        self.v3 ^= self.v2;
        self.v0 = Wrapping(self.v0.0.rotate_left(32));
        self.v2 += self.v1;
        self.v0 += self.v3;
        self.v1 = Wrapping(self.v1.0.rotate_left(17));
        self.v3 = Wrapping(self.v3.0.rotate_left(21));
        self.v1 ^= self.v2;
        self.v3 ^= self.v0;
        self.v2 = Wrapping(self.v2.0.rotate_left(32));
    }

}

/// Internal sip hash context used for cuckoo cycle computation.
#[derive(Debug)]
pub struct SipHashContext {
    state: SipHashState,
}

impl SipHashContext {

    pub fn new_with_prefix(prefix: &[u8]) -> Self {
        
        let hash_raw = Sha256::new_with_prefix(prefix).finalize();
        let hash = hash_raw.as_slice();
        let k0 = u64::from_le_bytes(hash[0..8].try_into().unwrap());
        let k1 = u64::from_le_bytes(hash[8..16].try_into().unwrap());

        Self {
            state: SipHashState {
                v0: Wrapping(k0 ^ 0x736f6d6570736575),
                v1: Wrapping(k1 ^ 0x646f72616e646f6d),
                v2: Wrapping(k0 ^ 0x6c7967656e657261),
                v3: Wrapping(k1 ^ 0x7465646279746573),
            },
        }

    }

    pub fn sip_hash24(&self, nonce: u64) -> u64 {
        
        let mut s = self.state.clone();

        s.v3 ^= nonce;
        s.round();
        s.round();
        s.v0 ^= nonce;
        s.v2 ^= 0xFF;
        s.round();
        s.round();
        s.round();
        s.round();
        
        (s.v0 ^ s.v1 ^ s.v2 ^ s.v3).0

    }

}
