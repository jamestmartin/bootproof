// Kernels cannot use the standard library
// because it depends on features like IO and memory allocation being available,
// but those features are *defined* by the kernel.
// The core and alloc crates form a subset of the standard library which you still can use;
// core only includes basic definitions which do not require anything special,
// and alloc only requires that you define your own global allocator, which we do.
#![no_std]
// The entry point to a UEFI application is called `efi_main`, not `main`.
#![no_main]
// Used because this is a UEFI application so we need to use its ABI to make calls.
#![feature(abi_efiapi)]
// Required by nightly when defining a global allocator.
#![feature(alloc_error_handler)]
#![feature(asm)]
// Used to conveniently define x86 interrupt handling routines.
#![feature(abi_x86_interrupt)]
#![feature(generic_associated_types)]
extern crate alloc;

mod arch;
mod driver;
mod graphics;
mod memory;
mod logger;

use alloc::vec::Vec;
use uefi::prelude::*;

// # Why did you choose to make bootproof a UEFI application?
//
// There are three major ways an operating system can choose to be loaded:
//
// 1. As a UEFI (Unified Extensible Firmware Interface) application.
//    UEFI will set up most hardware and the CPU in a simple sane way
//    (long mode, identity paged, etc.), and then load the UEFI app.
//    It also supports some decent drivers and features like a page allocator,
//    which are useful during booting but are not something you can depend on long-term
//    for reasons I'll get into later.
//    UEFI is implemented in firmware, which means it requires no installation or configuration;
//    just install your OS on your EFI partition, and it'll get loaded and work.
//
// 2. Through a bootloader, such as GRUB, in particular as a multiboot kernel.
//    This will also set up things in a sane way and load your OS,
//    and is generally more configurable and portable than UEFI
//    (in particular because it works on systems which do not support UEFI,
//    such as legacy, BIOS-only systems, and architectures which do not use it).
//
// 3. Through the BIOS. The BIOS will load your application however the CPU just happens to be,
//    i.e. 16-bit real mode, and provides a bunch of outdated drivers and information,
//    which are often missing so that multiple strategies are needed,
//    and are generally only available in 16-bit real mode.
//    It will only load a few KiB of your OS and beyond that you're on your own.
//
// The BIOS is a legacy P.o.S. system which is deprecated and likely to eventually be removed,
// so I have no interest in supporting it.
// Furthermore, I do not need the legacy compatibility or configurability of a bootloader,
// and its portability with initial system setup, and I'd have to do substantial work to port
// this OS to other platforms anyway, so I have chosen instead to use the
// native, zero-installation, zero-configuration solution of being loaded directly from UEFI.
//
// Support for being loaded as a multiboot kernel is
// something I'd be willing to have in the future,
// but it's not something I'm going to do until it becomes neccesary, if it ever even does.
#[entry]
fn efi_main(handle: Handle, st_boot: SystemTable<Boot>) -> Status {
    // UEFI applications are, from the perspective of the operating system, split into two phases:
    // a booting phase, where the UEFI boot services are available,
    // and the runtime phase, where they are not (although a few "runtime" services
    // are available in either phase).
    // (From the perspective of UEFI itself, there are more phases,
    // before and after the UEFI application's lifetime, but these are not relevant to us).
    // The UEFI boot services are the stuff like the allocator and various drivers.
    //
    // So if these UEFI boot services are so great, why should you leave them?
    // Because they interfere with you writing your own drivers and so forth.
    // The UEFI drivers are great if you're writing an application like a bootloader
    // or something that specifically needs full system control like a hardware tester,
    // but when you're trying to make a fully-featured operating system runtime,
    // you will need to use hardware in more sophisticated ways than UEFI allows.
    // There is no reliable way to make UEFI boot services continue to work
    // when you try to take control over memory, or interrupts, or anything else,
    // which makes it impossible to write your own drivers without disabling it.
    //
    // In our case, we already *have* been loaded by the UEFI, so we won't need *any*
    // of the UEFI's boot services (except for those which are inherently necessary to leave it,
    // i.e. the UEFI allocator), so our goal should be to leave it as quickly as possible,
    // so we can get on with setting up our hardware we'll actually need it.

    // First, we'll need to set Rust's global allocator to use the UEFI page allocator,
    // so we can allocate stuff until we get our own allocator working
    // (which we can't do until we've left boot services).
    //
    // This allocator is necessary for two reasons:
    //
    // 1. So we can allocate somewhere to store the UEFI memory maps,
    //    which is necessary to leave UEFI boot services,
    // 2. So we can allocate space for our runtime allocator's data structures.
    //
    // It has the additional benefit of allowing us to use the `println!` macro for debugging,
    // which depends on the `format!` macro, which allocates `String`s.
    //
    // I really wish I *didn't* depend on the UEFI allocator,
    // but I haven't found a good way around it so far.
    // (There *are* ways I can think of, but they're difficult enough to not be worth it.)
    use crate::memory::allocator::{ALLOCATOR, GlobalAllocator};
    unsafe {
        use crate::memory::allocator::uefi::UefiAllocator;
        // The allocator must be global and have a static lifetime because of how Rust works;
        // we do not have scoped allocators, only an application-wide global one.
        // We work around this by using a mutable global variable which we set to
        // use whichever implementation (boot or runtime) is available at the time.
        // The UEFI allocator needs to be able to reference the boot services table
        // so it can make allocations, and, being a global variable, it needs to be an owned copy.
        // This is unfortunate because the safety of the use of the boot table
        // depends on lifetimes, so that you cannot own a copy of the boot table
        // after you exit boot services, and cloning it violates that safety.
        // Trying to use the UEFI allocator after exiting boot services would be bad,
        // so I have to make this unsafe disclaimer:
        //
        // **DO NOT FORGET TO DISABLE THIS ALLOCATOR AFTER LEAVING UEFI BOOT SERVICES.**
        //
        // Furthermore, Rust doesn't *know* that which allocator I've used has changed
        // so it might try to free data which was allocated by a different allocator.
        // The runtime allocator has knowledge of what memory the UEFI boot services allocated
        // through the memory map the UEFI provides when you exit boot services.
        // However, I don't to require the allocator to keep track of that
        // in conjunction with the memory that it allocated itself,
        // so instead I'll make a second unsafe disclaimer:
        //
        // **ALL ALLOCATIONS MADE BY THE UEFI ALLOCATOR
        // MUST BE STATIC OR FREED BEFORE BOOT SERVICES EXITS.**
        ALLOCATOR = GlobalAllocator::Uefi(UefiAllocator::new(st_boot.unsafe_clone()));
    }

    unsafe {
        // For now, I use a serial device for logging kernel debug output.
        // Although serial ports don't physically exist on modern devices,
        // they're still supported by emulators, and they're extremely useful for debugging
        // thanks to their simplicity.
        // For QEMU, you can set `-serial stdio` and kernel output will be logged to STDOUT.
        use crate::driver::tty::serial::{COM1_PORT, SerialTty};
        logger::set_tty(SerialTty::new(COM1_PORT));
        logger::init().unwrap();
    }

    // Next we have to set up our runtime allocator and exit UEFI boot services.
    // These must be done simultaneously because our runtime allocator
    // depends on the UEFI memory map, which we get as a result of exiting boot services.
    // The memory map describes where a bunch of important stuff lies in memory,
    // and most importantly to us, describes what memory is free for allocation
    // what memory is currently in use by the kernel and UEFI runtime services,
    // and what memory is e.g. reserved by the CPU or contains memory-mapped devices.

    // We must provide a buffer (mmap_buf) for UEFI to write the memory map to.
    // We can't let it be de-allocated because it is allocated using the UEFI allocator,
    // for the reasons described above.
    let mut mmap_buf = Vec::new();
    let (_mmap, st) = {
        let bs = st_boot.boot_services();
        // A lot of allocations can happen between the buffer being allocated
        // and the buffer being populated when the boot services exit
        // (both by us and the UEFI's own processes;
        // in fact, reserving space is necessary even when you *immediately* load the memory map),
        // so we have to leave extra space in the memory map for those allocations.
        // 1024 is a number that I came up with by repeatedly testing numbers
        // until the kernel stopped crashing.
        mmap_buf.resize(bs.memory_map_size() + 1024, 0);

        // First we read the memory map so that the runtime allocator
        // can decide how much space it needs to allocate for its own data structures
        // using the UEFI allocator, which needs to be done before exiting UEFI boot services.
        // Between now and exiting boot services, only kernel and boot services will be made,
        // not changes to reserved memory and so forth (or at least I hope not!
        // so the amount of physical memory the allocator needs to keep track of will not change.
        use crate::memory::allocator::standard::StandardAllocator;
        let mut allocator;
        {
            let mut mmap = bs.memory_map(mmap_buf.as_mut_slice())
                .expect_success("Failed to exit the UEFI boot services.").1;
            allocator = StandardAllocator::new(&mut mmap);
        }

        // Actually exit UEFI boot services!
        let (st, mut mmap) = st_boot.exit_boot_services(handle, mmap_buf.as_mut_slice())
            .expect_success("Failed to exit the UEFI boot services.");

        // We now populate the allocator with the final memory map.
        // Before we were just allocating space for data structures,
        // but the actual memory used wasn't set in stone; now it is.
        // Since we don't distinguish boot services memory
        // from unallocated memory after exiting boot services,
        // perhaps we could just populate it from the original memory map and ignore this entirely?
        // I'm already making the assumption that reserved/runtime memory won't change,
        // and I don't make any new kernel allocations between then and now.
        allocator.populate(&mut mmap);
        unsafe { ALLOCATOR = GlobalAllocator::Standard(allocator); }

        (mmap, st)
    };

    // Now that UEFI is no longer handling interrupts,
    // we want them disabled until we set up our own handler,
    // which we will do... also right now.
    // Interrupt handling is necessary to write drivers for most devices.
    use x86_64::instructions::interrupts;
    interrupts::disable();

    use crate::arch::x86_64::{gdt, idt};
    // TODO: Resetting the GDT hasn't actually proven to be necessary in the emulator.
    //   However, I'm not sure if that's true in general,
    //   and at worst it seems harmless, so it stays for now.
    //   That said, further research is needed.
    gdt::load();
    idt::load();
    // We now have our own interrupt handler so we can re-enable them now.
    // That said, we still need to set up APIC to recieve interrupts for devices,
    // which isn't something that I've programmed yet. I'm working on it, though!
    interrupts::enable();

    // Everything up to this point has been setting up the CPU state, drivers, etc.
    // Now we begin running actual programs
    // (or in this case, since we don't support actual programs yet,
    // whatever debug stuff I want to run).
    main(st)
}

fn main(st: SystemTable<uefi::table::Runtime>) -> ! {
    // Put whatever code you want for debugging/testing purposes here...
    arch::x86_64::breakpoint();

    // There's nothing left for us to do at this point,
    // because there are no meaningful programs to run.
    // Instead, we'll just spin forever until the computer is turned off.
    // We don't want to shut down so we can continue displaying any debug output.
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
