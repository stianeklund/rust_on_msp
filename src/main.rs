#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate volatile_register;
use volatile_register::RW;

// PADIR & PAOUT are changed from the original as the MSP-EXP430FR4133 uses different pins
extern "C" {
    static mut WDTCTL:RW<u16>;
    static mut PBDIR_H: RW<u8>;
    static mut PBOUT_H: RW<u8>;
}

#[no_mangle]
#[link_section = "__interrupt_vector_reset"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = main;

pub unsafe extern "C" fn main() -> ! {
    // Turn Watchdog timer off (it's on by default)
    // WDTCTL.write(WDTPW | WDTHOLD);

    WDTCTL.write(0x5A00 + 0x80);
    PBDIR_H.write(0b0100_0001);
    PBOUT_H.write(0x01); // Turn LED on
    loop {
        PBOUT_H.modify(|x| !x);
        delay(40000);
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
