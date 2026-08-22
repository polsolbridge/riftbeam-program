#![no_std]
#![allow(unexpected_cfgs)]

use core::panic::PanicInfo;

// Solana BPF entrypoint: input ptr → 0 = SUCCESS
#[no_mangle]
pub extern "C" fn entrypoint(_input: *mut u8) -> u64 {
    0
}

#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    loop {}
}
