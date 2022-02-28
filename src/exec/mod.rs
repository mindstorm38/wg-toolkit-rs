//! Automatic disassembly of important information from game's executable.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use pelite::pe64::{Pe, PeFile, PeObject, headers::SectionHeaders};
use pelite::FileMap;

use iced_x86::{Code, Decoder, DecoderOptions, DecoderError, Mnemonic};


mod string;
use string::CstrFinderIterator;



pub struct Disassembly<'a> {
    pe: PeFile<'a>,
    addr_base: u64,
    strings: HashMap<u64, &'a str>,
    functions: HashMap<u64, ()>
}

impl<'a> Disassembly<'a> {

    pub fn new(data: &'a [u8]) -> Option<Self> {

        let pe = PeFile::from_bytes(data).ok()?;
        let addr_base = pe.optional_header().ImageBase as u64;

        Some(Self {
            pe,
            addr_base,
            strings: HashMap::new(),
            functions: HashMap::new()
        })

    }

    pub fn get_addr_base(&self) -> u64 {
        self.addr_base
    }

    pub fn get_sections(&self) -> &SectionHeaders {
        self.pe.section_headers()
    }

    pub fn get_strings(&self) -> &HashMap<u64, &'a str> {
        &self.strings
    }

    pub fn get_section_ptrs(&self, section_name: &[u8]) -> Option<(u64, usize, usize)> {
        self.pe.section_headers()
            .by_name(section_name)
            .map(|section| (
                section.VirtualAddress as u64,
                section.PointerToRawData as usize,
                section.PointerToRawData as usize + section.SizeOfRawData as usize
            ))
    }

    pub fn analyze_strings(&mut self, from_addr: u64, from: usize, to: usize) {
        let base = self.addr_base;
        self.strings.extend(CstrFinderIterator::new(&self.pe.image()[from..to])
            .map(move |(i, s)| (base + from_addr + i as u64, s)));
    }

    pub fn analyze_strings_in_section(&mut self, section_name: &[u8]) {
        let (from_addr, from, to) = self.get_section_ptrs(section_name).unwrap();
        self.analyze_strings(from_addr, from, to);
    }

    pub fn analyze_functions(&self, section_name: &[u8]) {

        let (from_addr, from, to) = self.get_section_ptrs(section_name).unwrap();
        let section_data = &self.pe.image()[from..to];
        let init_ip = self.addr_base + from_addr;

        let mut decoder = Decoder::with_ip(64, section_data, init_ip, DecoderOptions::NONE);

        #[derive(Eq, PartialEq, Hash)]
        struct FunctionCall {
            from_pos: usize,
            from_ip: u64,
            to_ip: u64
        }

        let mut call_addrs = HashSet::new();

        loop {

            let instruction_pos = decoder.position();
            let instruction = decoder.decode();

            match (instruction.code(), instruction.mnemonic(), decoder.last_error()) {
                (Code::INVALID, _, DecoderError::NoMoreBytes) => break,
                (_, Mnemonic::Mov, _) => {

                },
                (c, _, _) if c.is_call_near() => {
                    call_addrs.insert(FunctionCall {
                        from_pos: instruction_pos,
                        from_ip: instruction.ip(),
                        to_ip: instruction.near_branch_target()
                    });
                },
                _ => {}
            }

        }

        println!("Unique calls count: {}", call_addrs.len());
        //println!("{call_addrs:#?}");

    }

}
