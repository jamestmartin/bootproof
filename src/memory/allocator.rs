pub mod standard;
pub mod uefi;

use alloc::alloc::GlobalAlloc;
use core::alloc::Layout;

pub enum GlobalAllocator {
    None,
    Standard(standard::StandardAllocator),
    Uefi(uefi::UefiAllocator),
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self {
            GlobalAllocator::None => panic!("No allocator available!"),
            GlobalAllocator::Uefi(alloc) => alloc.alloc(layout),
            GlobalAllocator::Standard(alloc) => alloc.alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match self {
            GlobalAllocator::None => panic!("No allocator available!"),
            GlobalAllocator::Uefi(alloc) => alloc.dealloc(ptr, layout),
            GlobalAllocator::Standard(alloc) => alloc.dealloc(ptr, layout),
        }
    }
}

#[global_allocator]
pub static mut ALLOCATOR: GlobalAllocator = GlobalAllocator::None;

#[alloc_error_handler]
fn handle_error(layout: Layout) -> ! {
    panic!("Failed to allocate {:?}", layout);
}
