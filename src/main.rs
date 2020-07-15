#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]

mod logger;
mod misc;

use crate::logger::{LoggerBackend, LOGGER_BACKEND};
use crate::misc::halt;

use core::mem;
use core::slice;
use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};

fn setup(st: &SystemTable<Boot>, _handle: Handle) {
    st.stdout().reset(false).expect_success("Failed to reset UEFI stdout.");

    println!("Booting...");
}

fn main(_st: SystemTable<uefi::table::Runtime>, _mmap: uefi::table::boot::MemoryMapIter) -> ! {
    halt();
}

#[entry]
fn efi_main(handle: Handle, st_boot: SystemTable<Boot>) -> Status {
    // Tasks that require the UEFI boot services.

    unsafe {
        LOGGER_BACKEND = LoggerBackend::UefiStdio(st_boot.unsafe_clone());
    }

    setup(&st_boot, handle);

    // Exit the UEFI boot services.

    // The memory map buffer must live at least as long as the memory map iterator.
    let mmap_buf;
    let (st_runtime, mmap) = {
        let bs = st_boot.boot_services();

        // More allocations can happen between the allocation of the buffer and the buffer being filled,
        // so arbitrarily add space for 8 more memory descriptors just to make sure there's enough.
        let mmap_buf_size = bs.memory_map_size() + 8 * mem::size_of::<MemoryDescriptor>();
        mmap_buf = bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, mmap_buf_size)
            .expect_success("Could not allocate space for UEFI memory map.");
        let mmap_buf_slice = unsafe { slice::from_raw_parts_mut(mmap_buf as *mut u8, mmap_buf_size) };
        st_boot.exit_boot_services(handle, mmap_buf_slice).expect_success("Failed to exit the UEFI boot services.")
    };

    // Tasks that do not require the UEFI boot services.

    // I do not currently have an adequate stdout for post-UEFI, but the UEFI one is now invalid.
    unsafe {
        LOGGER_BACKEND = LoggerBackend::None;
    }

    main(st_runtime, mmap);
}
