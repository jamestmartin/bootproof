#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(generic_associated_types)]
extern crate alloc;

mod allocator;
#[macro_use]
mod graphics;
mod arch;

use core::mem;
use core::slice;
use crate::allocator::{Allocator, ALLOCATOR};
use crate::graphics::tty::{Tty, STDOUT, STDERR};
use crate::graphics::tty::serial::SerialTty;
use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};

#[entry]
fn efi_main(handle: Handle, st_boot: SystemTable<Boot>) -> Status {
    // Although serial ports don't physically exist on modern devices,
    // they're still supposed by emulators (for QEMU you can set `-serial stdio`),
    // and they're extremely useful for debugging
    // because they don't require any setup and are trivial to use.
    unsafe {
        STDOUT = Some(SerialTty::new(0x3F8));
        STDERR = Some(SerialTty::new(0x3F8));
    }

    // Our first task is to exit the UEFI boot services.
    // UEFI provides a whole bunch of useful device drivers,
    // but they're unusable after you exit the boot services,
    // and you absolutely *have* to exit boot services to do most OS-related things.

    // When we exit boot services, UEFI provides us with a memory map,
    // which describes where a bunch of important stuff lies in memory
    // (e.g. memory-mapped devices and the ACPI tables),
    // and what memory is available for us to use safely.
    let (st, mmap) = {
        // We must provide a buffer (mmap_buf) for UEFI to write the memory map to.
        let bs = st_boot.boot_services();

        // More allocations can happen between the allocation of the buffer and the buffer being filled,
        // so add space for 8 more memory descriptors (an arbitrary number) just to make sure there's enough.
        let mmap_buf_size = bs.memory_map_size() + 8 * mem::size_of::<MemoryDescriptor>();

        // In the memory map, the OS's own code and data is included as LOADER_CODE and LOADER_DATA.
        // This is good to know so we don't accidentally write over it!
        // We allocate the mmap_buf as LOADER_DATA for this purpose.
        let mmap_buf = bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, mmap_buf_size)
            .expect_success("Failed to allocate memory for UEFI memory map buffer.");

        // allocate_pages returns a u64 address, but we need a `&mut [u8]`.
        let mmap_buf_slice = unsafe { slice::from_raw_parts_mut(mmap_buf as *mut u8, mmap_buf_size) };

        // Finally, we actually exit the UEFI boot services.
        st_boot.exit_boot_services(handle, mmap_buf_slice).expect_success("Failed to exit the UEFI boot services.")
    };

    // Set up the stuff I need to handle interrupts, which is necessary to write drivers for most devices.
    use x86_64::instructions::interrupts;
    use crate::arch::x86_64::{gdt, idt};

    interrupts::disable();
    // TODO: I've actually found that resetting the GDT isn't necessary in the emulator.
    //   However, I'm not sure if that's true in general, and at worst it seems harmless, so it stays for now.
    //   That said, further research is needed.
    gdt::load();
    idt::load();
    interrupts::enable();

    // Everything up to this point has been setting up the CPU state, drivers, etc.
    // Now we begin running actual programs
    // (or in this case, since we don't support actual programs yet, whatever debug stuff I want to run).
    main(st, mmap)
}

fn main(_st: SystemTable<uefi::table::Runtime>, _mmap: uefi::table::boot::MemoryMapIter) -> ! {
    // Put whatever code you want for debugging/testing purposes here...
    arch::x86_64::breakpoint();

    // There's nothing left for us to do at this point, because there are no meaningful programs to run.
    // Instead, we'll just spin forever until the computer is turned off.
    // We do *not* disable interrupts to allow for testing the interrupt handlers.
    loop { x86_64::instructions::hlt(); }
}
