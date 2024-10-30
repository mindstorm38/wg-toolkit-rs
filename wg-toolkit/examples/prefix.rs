use wgtk::util::cuckoo::CuckooContext;


fn main() {

    // let offset = 0u32;

    let p0 = 0xC5FF0000;
    let p1 = 0xFC000000;

    for offset in [0x6B54BB86, 0x38131C1F, 0x918033AC, 0x7A11751F] {
        println!("prefix: {:08X}", calc_prefix(offset, p0, p1));
    }

    let mut log_counter = 0u32;
    for offset in 0..u32::MAX {
        
        if log_counter == 0 {
            println!("progress: {}%", offset as f32 / u32::MAX as f32 * 100.0);
        }

        log_counter += 1;
        if log_counter > u32::MAX / 1000 {
            log_counter = 0;
        }
        
        if calc_prefix(offset, p0, p1) == 0x64C20486 {
            println!("offset: 0x{offset:08X}");
        }
    }

    // Searching how we can get this value: 64C20486

}

#[inline]
fn calc_prefix(offset: u32, p0: u32, p1: u32) -> u32 {
    let a = offset.wrapping_add(p0).wrapping_add(p1);
    let b = a << 13;
    let c = (b ^ a) >> 17;
    let d = c ^ b ^ a ^ ((c ^ b ^ a) << 5);
    d
}
