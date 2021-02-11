#![no_main]
#![no_std]
#![feature(llvm_asm)]
#![cfg_attr(feature = "verified", feature(default_alloc_error_handler))]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[cfg(not(feature = "verified"))]
mod riotboot;

#[cfg(not(feature = "verified"))]
extern crate fletcher;

// HACKSPEC specifics start
#[cfg(feature = "verified")]
extern crate alloc;

#[cfg(feature = "verified")]
mod riotboot {
    use alloc::vec::Vec;

    pub use hacspec_lib::seq::Seq;
    pub use hacspec_riot_bootloader::Header;

    pub fn choose_image(images: &[&Header]) -> Option<u32> {
        let images = images.iter().map(|x| **x).collect::<Vec<_>>();
        let images = Seq::from_vec(images);
        let (have, addr) = hacspec_riot_bootloader::choose_image(images);
        if have {
            Some(addr)
        } else {
            None
        }
    }
}

#[cfg(feature = "verified")]
use static_alloc::Bump;

#[cfg(feature = "verified")]
#[global_allocator]
static A: Bump<[u8; 1 << 14]> = Bump::uninit();

// HACKSPEC specifics end

use riotboot::{choose_image, Header};

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub extern "C" fn cpu_jump_to_image(image_address: u32) -> ! {
    unsafe {
        llvm_asm!("
    ldr     r1, [$0]        /* r1 = *image_address          */
    msr     msp, r1         /* MSP = r1                     */
    ldr     $0, [$0, #4]    /* r0 = *(image_address + 4)    */
    orr.w   $0, $0, #1      /* r0 |= 0x1 (set thumb bit)    */
    bx      $0              /* branch to image              */
        "
        :
        : "{r0}"(image_address)
        : "r0"
        : "volatile"
        );
        loop {}
    }
}

#[entry]
fn main() -> ! {
    let headers = unsafe { [&*(0x2000 as *const Header), &*(528_384 as *const Header)] };
    let image = choose_image(&headers[..]);

    if let Some(image_address) = image {
        //hprintln!("jumping to image address {:#08x}", image_address).unwrap();
        cpu_jump_to_image(image_address);
    }
    panic!("no suitable image found");
}
