#![no_main]
#![no_std]
#![feature(asm)]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use nrf52840_hal::gpio::Level;
use particle_xenon::{prelude::*, Board};

pub mod fletcher;
mod riotboot;
use riotboot::{choose_image, Header};

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub extern "C" fn cpu_jump_to_image(image_address: u32) -> ! {
    unsafe {
        asm!("
    /* r1 = *image_address      */
    ldr    r1, [$0]
    /* MSP = r1                 */
    msr msp, r1
    /* r0 = *(image_address + 4) */
    ldr    $0, [$0, #4]
    /* r0 |= 0x1 (set thumb bit) */
    orr.w   $0, $0, #1
    /* branch to image */
    bx $0
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
    let headers = unsafe { [&*(0x2000 as *const Header), &*(528384 as *const Header)] };
    let image = choose_image(&headers[..]);

    if let Some(image_address) = image {
        //hprintln!("jumping to image address {:#08x}", image_address).unwrap();
        cpu_jump_to_image(image_address);
    }
    panic!("no suitable image found");
}
