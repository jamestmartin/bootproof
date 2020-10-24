use alloc::alloc::GlobalAlloc;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::cell::UnsafeCell;
use uefi::table::boot::MemoryDescriptor;

// TODO: Support granularity better than pages.
// TODO: Use an allocation algorithm that isn't absolute garbage!!

/// **This allocator only supports page-level granularity.**
/// Be careful not to use it for small allocations.
pub struct StandardAllocator {
    pages: UnsafeCell<Vec<u8>>,
}

const PAGE_SIZE: usize = 4096;

pub fn get_bit(bytes: &[u8], index: usize) -> bool {
    let (byte, bit) = num_integer::div_rem(index, 8);
    let mask = 0b10000000u8 >> bit;
    bytes[byte] & mask > 0
}

pub fn set_bit(bytes: &mut [u8], index: usize, value: bool) {
    let (byte, bit) = num_integer::div_rem(index, 8);
    let mask = 0b10000000u8 >> bit;
    if value {
        bytes[byte] |= mask;
    } else {
        bytes[byte] &= !mask;
    }
}

impl StandardAllocator {
    /// Allocates a new allocator data structure sufficient
    /// to use the physical memory provided in the memory map.
    /// With what allocator does the allocator get allocated? The UEFI allocator.
    /// This does *not* pre-populate the allocator with usage data;
    /// by default, it will behave as though every page were allocated.
    /// Use `populate` to fill the allocator with actual data
    /// using the map that UEFI provides when you exit boot services.
    pub fn new<'buf>(mmap: &mut impl ExactSizeIterator<Item = &'buf MemoryDescriptor>) -> StandardAllocator {
        // Try to find the largest physical address
        // and create a bitmap allowing the allocation of that much memory.
        let greatest_physical_page =
            mmap.map(|d| num_integer::div_ceil(d.phys_start as usize, PAGE_SIZE) + d.page_count as usize).max()
                .unwrap();

        let mut pages = Vec::with_capacity(num_integer::div_ceil(greatest_physical_page, 8));
        // I can fit up to 8 pages in a byte in my bitmap.
        pages.resize(num_integer::div_ceil(greatest_physical_page, 8), 0xFF);

        StandardAllocator {
            pages: UnsafeCell::new(pages)
        }
    }

    pub fn populate<'buf>(&mut self, mmap: &mut impl ExactSizeIterator<Item = &'buf MemoryDescriptor>) {
        let self_pages = unsafe { &mut *self.pages.get() };
        // Mark all unsable memory as free for allocations.
        for entry in mmap {
            use uefi::table::boot::MemoryType;
            if entry.ty == MemoryType::BOOT_SERVICES_CODE
                || entry.ty == MemoryType::BOOT_SERVICES_DATA
                || entry.ty == MemoryType::CONVENTIONAL {
                let base = entry.phys_start as usize / PAGE_SIZE;
                for offset in 0..entry.page_count as usize {
                    set_bit(self_pages.as_mut_slice(), base + offset, false);
                }
            }
        }

        // Even if the zero address is valid memory, we *definitely* don't want to allocate it.
        set_bit(self_pages.as_mut_slice(), 0, true);
    }

    pub fn free(&self) -> usize {
        let self_pages = unsafe { &mut *self.pages.get() };
        let mut free = 0;
        for page in 0..self_pages.len() * 8 {
            if !get_bit(self_pages.as_slice(), page) {
                free += 1;
            }
        }
        free
    }

    // Not very accurate because this includes a lot of reserved/unusable memory.
    pub fn used(&self) -> usize {
        let free = self.free();
        // This line of code crashes QEMU for inexplicable reasons.
        // I tried to figure out why and failed.
        let self_pages = unsafe { &*self.pages.get() };
        free - self_pages.as_slice().len() * 8
    }
}

unsafe impl GlobalAlloc for StandardAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let min_pages = num_integer::div_ceil(layout.size(), PAGE_SIZE);
        let mut pages = None;
        let self_pages = &mut *self.pages.get();
        for i in 0..(self_pages.len() * 8) {
            if !get_bit(&self_pages.as_slice(), i) {
                pages = match pages {
                    Some((begin, size)) => Some((begin, size + 1)),
                    None => Some((i, 1)),
                };

                if pages.unwrap().1 >= min_pages {
                    break;
                }
            } else {
                pages = None;
            }
        }

        if pages.is_none() || pages.unwrap().1 < min_pages {
            panic!("Not enough contiguous memory!");
        }

        let (begin, size) = pages.unwrap();

        let address = begin * PAGE_SIZE;

        for offset in 0..size {
            set_bit(self_pages.as_mut_slice(), begin + offset, true);
        }

        address as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let begin = ptr as usize / PAGE_SIZE;
        let size = num_integer::div_ceil(layout.size(), PAGE_SIZE);
        let self_pages = &mut *self.pages.get();

        for offset in 0..size {
            set_bit(self_pages.as_mut_slice(), begin + offset, false);
        }
    }
}
