#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(generic_associated_types)]
extern crate alloc;

mod allocator;
mod arch;
mod graphics;

mod logger;

use alloc::vec::Vec;
use crate::graphics::tty::serial::SerialTty;
use uefi::prelude::*;

#[entry]
fn efi_main(handle: Handle, st_boot: SystemTable<Boot>) -> Status {
    use crate::allocator::{ALLOCATOR, GlobalAllocator};
    unsafe {
        // Generally speaking, we want to depend on UEFI as little as possible,
        // so the need for a UEFI allocator may seem a bit strange.
        // However, there's this awkward time during booting when we need
        // to allocate space for our "real" allocator's data structures and the UEFI memory maps,
        // the result being that we need an allocator for our allocator.
        // In theory there are probably ways to get around it, but why bother?
        // Just taking advantage of the UEFI allocator briefly is a lot easier.
        // (This also lets us use `println!` prior to our main allocator being set up.)
        use crate::allocator::uefi::UefiAllocator;
        // ABSOLUTELY DO NOT FORGET TO DISABLE THIS AFTER LEAVING UEFI BOOT SERVICES.
        // ALL ALLOCATIONS MUST BE STATIC OR BE FREED BEFORE BOOT SERVICES EXITS.
        // If the're not, Rust still try to free UEFI-allocated data using the new allocator,
        // which is undefined behavior.
        ALLOCATOR = GlobalAllocator::Uefi(UefiAllocator::new(st_boot.unsafe_clone()));

        logger::set_tty(SerialTty::new(0x3F8));
        logger::init().unwrap();
    }

    // Our first task is to exit the UEFI boot services.
    // UEFI provides a whole bunch of useful device drivers,
    // but they're unusable after you exit the boot services,
    // and you absolutely *have* to exit boot services to do most OS-related things.

    // When we exit boot services, UEFI provides us with a memory map,
    // which describes where a bunch of important stuff lies in memory
    // (e.g. memory-mapped devices and the ACPI tables),
    // and what memory is available for us to use safely.

    // We can't let the memory map be de-allocated because it is allocated using the UEFI allocator,
    // but would end up being freed using the standard allocator, which is undefined behavior.
    // We must provide a buffer (mmap_buf) for UEFI to write the memory map to.
    let mut mmap_buf = Vec::new();
    let (_mmap, st) = {
        let bs = st_boot.boot_services();
        // More allocations can happen between the allocation of the buffer and the buffer being filled,
        // so add space for 32 more memory descriptors (an arbitrary number) just to make sure there's enough.
        mmap_buf.resize(bs.memory_map_size() + 1024, 0);

        // HACK: I hate having to use the UEFI allocator just to set up another allocator!
        //   There's got to be a better way.
        use crate::allocator::standard::StandardAllocator;
        let mut allocator;
        {
            let mut mmap = bs.memory_map(mmap_buf.as_mut_slice())
                .expect_success("Failed to exit the UEFI boot services.").1;
            allocator = StandardAllocator::new(&mut mmap);
        }

        // Finally, we actually exit the UEFI boot services.
        let (st, mut mmap) = st_boot.exit_boot_services(handle, mmap_buf.as_mut_slice())
            .expect_success("Failed to exit the UEFI boot services.");

        allocator.populate(&mut mmap);
        unsafe { ALLOCATOR = GlobalAllocator::Standard(allocator); }

        (mmap, st)
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
    main(st)
}

fn main(_st: SystemTable<uefi::table::Runtime>) -> ! {
    // Put whatever code you want for debugging/testing purposes here...
    arch::x86_64::breakpoint();

    // There's nothing left for us to do at this point, because there are no meaningful programs to run.
    // Instead, we'll just spin forever until the computer is turned off.
    // We do *not* disable interrupts to allow for testing the interrupt handlers.
    loop { x86_64::instructions::hlt(); }
}

#[macro_export]
macro_rules! panic {
    ($($arg:expr),*) => {{
        log::error!($($arg),*);
        // FIXME: Panic shouldn't depend on an architecture-specific function.
        crate::arch::x86_64::halt()
    }}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic!("{}", info);
}
