use alloc::alloc::GlobalAlloc;
use core::alloc::Layout;
use uefi::prelude::{Boot, SystemTable};
use uefi::table::boot::{AllocateType, MemoryType};

/// **This allocator only supports page-level granularity.**
/// Be careful not to use it for small allocations.
/// In particular, it should only be used to allocate data structures
/// for the purpose of setting up another, better allocator.
pub struct UefiAllocator {
    // We must directly store an owned system table because:
    //   1. It is impossible to take ownership of ST boot services, and
    //   2. we cannot store a reference to *anything* here
    //      because the global allocator has to be static even if we know it's really not.
    st: SystemTable<Boot>,
}

impl UefiAllocator {
    pub fn new(st: SystemTable<Boot>) -> UefiAllocator {
        UefiAllocator { st: st }
    }
}

const PAGE_SIZE: usize = 4096;

unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.st.boot_services()
            .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA,
                            num_integer::div_ceil(layout.size(), PAGE_SIZE))
            .expect("Failed to allocate memory!").unwrap()
            as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.st.boot_services().free_pages(ptr as u64, num_integer::div_ceil(layout.size(), PAGE_SIZE))
            .expect("Failed to free memory!").unwrap();
    }
}
