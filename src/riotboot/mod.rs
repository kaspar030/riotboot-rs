extern crate fletcher;
//use cortex_m_semihosting::{dbg, hprintln};
use fletcher::Fletcher32;

/// "RIOT"
const RIOTBOOT_MAGIC: u32 = 0x544f_4952;

/// struct defining riotboot header
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    /// Header magic number (always "RIOT")
    magic_number: u32,
    /// Integer representing the partition version
    sequence_number: u32,
    /// Address after the allocated space for the header
    start_addr: u32,
    /// Checksum of riotboot_hdr
    checksum: u32,
}

impl Header {
    fn as_u16_slice(&self) -> &[u16] {
        unsafe {
            ::core::slice::from_raw_parts(
                (self as *const Header) as *const u16,
                (::core::mem::size_of::<Header>() / 2) - 2, // assume checksum is last and two words long, and must be skipped.
            )
        }
    }
    pub fn is_valid(&self) -> bool {
        //hprintln!("magic: {:#08x}", self.magic_number).unwrap();
        if self.magic_number == RIOTBOOT_MAGIC {
            let mut fletcher = Fletcher32::new();
            fletcher.update(self.as_u16_slice());
            let sum = fletcher.value();
            //hprintln!("sum: {:#08x}", sum).unwrap();
            //hprintln!("expexted: {:#08x}", self.checksum).unwrap();
            return sum == self.checksum;
        }
        false
    }
}

pub fn choose_image(images: &[&Header]) -> Option<u32> {
    let mut image: Option<&Header> = None;

    for header in images {
        if header.is_valid() {
            if let Some(image) = image {
                if header.sequence_number <= image.sequence_number {
                    continue;
                }
            }
            //hprintln!("found image").unwrap();
            //hprintln!("valid image address: {:#08x}", header.start_addr).unwrap();
            image = Some(header)
        }
    }

    image.map_or(None, |x| Some(x.start_addr))
}

// #![test]
// mod test {
//     use super::*;

//     #[test]
//     test_
