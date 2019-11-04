#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vram = 0xa0000 as *mut u8;

    for i in 0..320*200 {
        unsafe {
            *vram.offset(i) = (i & 0x0f) as u8;
        }
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
