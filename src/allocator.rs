use alloc::alloc::GlobalAlloc;
use core::alloc::Layout;
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};

pub enum Allocator {
    None,
    Uefi(uefi::prelude::SystemTable<uefi::prelude::Boot>)
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self {
            Allocator::Uefi(st) => {
                crate::log!("Allocate {:?}", layout);
                st.boot_services().allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, layout.size())
                    .expect("Failed to allocate memory!")
                    .expect("Failed to allocate memory! 2")
                    as *mut u8
            },
            Allocator::None => panic!("No allocator available!")
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match self {
            Allocator::Uefi(st) => {
                crate::log!("Free {:?}", layout);
                st.boot_services().free_pages(ptr as u64, layout.size());
            },
            Allocator::None => {
                panic!("No allocator available!");
            }
        }
    }
}

#[global_allocator]
pub static mut ALLOCATOR: Allocator = Allocator::None;

#[alloc_error_handler]
fn handle_error(layout: Layout) -> ! {
    panic!("Failed to allocate {:?}", layout);
}
