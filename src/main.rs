#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::writeln;
use uefi::prelude::*;

// Required for use by the panic handler.
// This should not be used anywhere else.
static mut global_st: Option<*mut SystemTable<Boot>> = None;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let st = unsafe { &global_st.unwrap().read() };
    writeln!(st.stderr(), "stderr: {}", info);
    writeln!(st.stdout(), "stdout: {}", info);
    
    loop {}
}

#[entry]
fn efi_main(handle: Handle, st: SystemTable<Boot>) -> Status {
    let mut g_st = unsafe { st.unsafe_clone() };
    unsafe {
        global_st = Some(&mut g_st);
    }
    
    st.stdout().reset(false).expect_success("Failed to reset UEFI stdout.");
    writeln!(st.stdout(), "Hello, world!");
    
    loop {}
        
    Status::SUCCESS
}