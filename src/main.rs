#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
extern crate alloc;

mod allocator;
#[macro_use]
mod graphics;
mod misc;
mod arch;

use core::mem;
use core::slice;
use crate::allocator::{Allocator, ALLOCATOR};
use crate::graphics::tty::{Tty, STDOUT, STDERR};
use crate::graphics::tty::serial::SerialTty;
use crate::misc::halt;
use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};

fn setup(st: &SystemTable<Boot>, _handle: Handle) {
    st.stdout().reset(false).expect_success("Failed to reset UEFI stdout.");

    println!("Booting...");

    for entry in st.config_table() {
        use uefi::table::cfg::*;
        if entry.guid == ACPI2_GUID {
            print!("ACPI2");
        } else if entry.guid == SMBIOS_GUID {
            print!("SMBIOS");
        } else if entry.guid == SMBIOS3_GUID {
            print!("SMBIOS3");
        } else {
            print!("{}", entry.guid);
        }
        println!(": 0x{:016X}", entry.address as u64);
    }

    graphics::do_graphics(st);
}

fn main(_st: SystemTable<uefi::table::Runtime>, _mmap: uefi::table::boot::MemoryMapIter) -> ! {
    crate::arch::x86_64::breakpoint();

    halt()
}

#[entry]
fn efi_main(handle: Handle, st_boot: SystemTable<Boot>) -> Status {
    // Tasks that require the UEFI boot services.

    unsafe {
        ALLOCATOR = Allocator::Uefi(st_boot.unsafe_clone());
        STDOUT = Some(SerialTty::new(0x3F8));
        STDERR = Some(SerialTty::new(0x3F8));
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

    // Replace the GDT and IDT with my own so I can start handling interrupts.
    use x86_64::instructions::interrupts;
    use crate::arch::x86_64::{gdt, idt};

    interrupts::disable();
    gdt::load();
    idt::load();
    interrupts::enable();

    // Tasks that do not require the UEFI boot services.

    unsafe {
        // TODO: An allocator that works out of UEFI mode.
        ALLOCATOR = Allocator::None;
    }

    main(st_runtime, mmap)
}
