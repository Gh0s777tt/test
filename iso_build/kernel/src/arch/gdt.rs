//! GDT (Global Descriptor Table) Implementation
//! Provides segment descriptor management

use spin::Mutex;
use x86_64::instructions::segmentation::{Segment, CS, DS, ES, SS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

/// Double fault stack size
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// TSS with interrupt stack
static mut TSS: TaskStateSegment = TaskStateSegment::new();

/// GDT with entries
static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

/// Segment selectors
pub struct Selectors {
    pub code: SegmentSelector,
    pub data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}

/// Global selectors
static SELECTORS: Mutex<Option<Selectors>> = Mutex::new(None);

/// Initialize GDT
pub fn init() {
    use core::ptr::addr_of_mut;
    
    // SAFETY: We only modify TSS and GDT during initialization before they are loaded
    unsafe {
        // Set up TSS interrupt stack
        static mut STACK: [u8; 4096] = [0; 4096];
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = 
            x86_64::VirtAddr::from_ptr(addr_of_mut!(STACK));
        
        // Add entries to GDT
        let code = GDT.add_entry(Descriptor::kernel_code_segment());
        let data = GDT.add_entry(Descriptor::kernel_data_segment());
        let user_code = GDT.add_entry(Descriptor::user_code_segment());
        let user_data = GDT.add_entry(Descriptor::user_data_segment());
        let tss_selector = GDT.add_entry(Descriptor::tss_segment(&TSS));
        
        // Store selectors
        *SELECTORS.lock() = Some(Selectors {
            code,
            data,
            user_code,
            user_data,
            tss: tss_selector,
        });
        
        // Load GDT
        GDT.load();
        
        // Load segment registers
        CS::set_reg(code);
        DS::set_reg(data);
        ES::set_reg(data);
        SS::set_reg(data);
        
        // Load TSS
        load_tss(tss_selector);
    }
}