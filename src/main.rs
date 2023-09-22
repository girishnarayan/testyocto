
// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points



use::core::arch::asm;
use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;
    global_asm!(
        ".section .text._start"
    );

}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    unsafe{
        core::ptr::write_volatile(0x3F20_0008 as *mut u32,1<<3);
        
        
        core::ptr::write_volatile(0x3F20_001C as *mut u32,1<<21);

        for _ in 1..50000{
            asm!("nop");
        }
        core::ptr::write_volatile(0x3F20_0028 as *mut u32,1<<21);
        for _ in 1..50000{
            asm!("nop");
        }
    }


    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}



