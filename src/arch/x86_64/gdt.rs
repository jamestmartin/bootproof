use x86_64::instructions::segmentation::{load_ss, set_cs};
use x86_64::structures::gdt::{Descriptor, DescriptorFlags, GlobalDescriptorTable};

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

fn kernel_data_segment() -> Descriptor {
    use self::DescriptorFlags as Flags;

    let flags = Flags::USER_SEGMENT | Flags::PRESENT | Flags::WRITABLE;
    Descriptor::UserSegment(flags.bits())
}

pub fn load() {
    unsafe {
        let cs = GDT.add_entry(Descriptor::kernel_code_segment());
        GDT.add_entry(Descriptor::user_code_segment());
        GDT.add_entry(Descriptor::user_data_segment());
        let ss = GDT.add_entry(kernel_data_segment());
        GDT.load();
        set_cs(cs);
        load_ss(ss);
    }
}
