use alloc::alloc::GlobalAlloc;
use core::alloc::Layout;

pub enum Allocator {
    None,
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self {
            Allocator::None => panic!("No allocator available!")
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match self {
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
