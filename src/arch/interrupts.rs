use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use core::sync::atomic::{AtomicU8, Ordering};
use x86_64::instructions::port::Port;
use crate::arch::{gdt, time};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = 
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub static LAST_KEY: AtomicU8 = AtomicU8::new(0);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[PIC_1_OFFSET].set_handler_fn(timer_interrupt_handler);
        idt[PIC_1_OFFSET + 1].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init() { 
    IDT.load();
    

    unsafe {
        PICS.lock().initialize();
        
        // Explicitly unmask IRQ 0 (Timer) and IRQ 1 (Keyboard) on the Master PIC.
        // write to port 0x21 (Master PIC data port). 
        // Clearing bits 0 and 1 (writing 0xfc) allows timer and keyboard interrupts through.
        let mut master_mask = Port::<u8>::new(0x21);
        let current_mask = master_mask.read();
        master_mask.write(current_mask & !0x03); // Clear bits 0 and 1
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Add 1 millisecond to global clock
    time::TICKS.fetch_add(1, Ordering::Relaxed);
    
    // Tell the PIC crate we are done
    unsafe { PICS.lock().notify_end_of_interrupt(PIC_1_OFFSET); }
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    

    LAST_KEY.store(scancode, Ordering::Relaxed);
    
    unsafe { PICS.lock().notify_end_of_interrupt(PIC_1_OFFSET + 1); }
}