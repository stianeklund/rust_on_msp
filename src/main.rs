#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate volatile_register;
use volatile_register::RW;

// There are some differences between the MSP430G2553 & the MSP430FR4133
// Noticeably, the FR4133 does not have P1, P2, P3 etc, but rather PAOUT, PAOUT_L etc.
// Please refer to the symbols file.

extern "C" {
    static mut WDTCTL: RW<u16>;
    static mut PBDIR_H: RW<u8>;
    static mut PBOUT_H: RW<u8>;
    static mut PADIR: RW<u8>;
    static mut PAOUT: RW<u8>;
    static mut PM5CTL0: RW<u16>;
}

#[no_mangle]
#[link_section = "__interrupt_vector_reset"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = main;

pub unsafe extern "C" fn main() -> ! {
    // Turn Watchdog timer off (it's on by default)
    WDTCTL.write(0x5A00 + 0x0080);
    PBDIR_H.write(0b0100_0001);
    PADIR.write(0b0100_0001);

    // Disable GPIO power-on default
    // This locks the I/O pin configuration change (to / from) LPM5
    PM5CTL0.write(0x0130);
    // Turn LED on
    PAOUT.write(0x01);
    PBOUT_H.write(0x01);
    loop {
        PBOUT_H.modify(|x| !x);
        delay(50000);
        PAOUT.modify(|x| !x);

    }
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn delay(mut n: u16) {
    unsafe {
        asm! {
            "1: \n dec $0 \n jne 1b" : "+r" (n) ::: "volatile"
        }
    }
}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
