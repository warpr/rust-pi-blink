#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // In GPFSEL1 (function register 1),
        // write value 001 (GPIO pin is output)
        // to bits 26-24 (FSEL18 function select 18).
        core::ptr::write_volatile(0x3f20_0004 as *mut i32, 1 << 24);

        loop {
            // GPIO pin output set 0, set bit 18 to turn pin 18 on
            core::ptr::write_volatile(0x3f20_001c as *mut i32, 1 << 18);

            for _ in 1..50000 {
                asm!("nop");
            }

            // GPIO pin output set 0, clear bit 18 to turn pin 18 off
            core::ptr::write_volatile(0x3f20_0028 as *mut i32, 1 << 18);

            for _ in 1..50000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
