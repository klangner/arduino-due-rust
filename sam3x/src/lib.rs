#![no_std]

// This table is used to initialize MCU
pub struct VectorTable {
    pub reset_handler               : fn()->!,
    pub other_interrupt_vectors     : [u32; 44],
}

//unsafe impl Sync for VectorTable {}


// The actual vector table.
//#[link_section=".vectors"]
//pub static VECTOR_TABLE: VectorTable = unsafe {
//    VectorTable {
//        reset_handler              : reset_handler,
//        other_interrupt_vectors    : [0; 44],
//    }
//};


// Addresses of several registers used to control parallel I/O.
pub const PB_PIO_ENABLE       : *mut u32 = 0x400E1000 as *mut u32;
pub const PB_OUTPUT_ENABLE    : *mut u32 = 0x400E1010 as *mut u32;
pub const PB_SET_OUTPUT_DATA  : *mut u32 = 0x400E1030 as *mut u32;
pub const PB_CLEAR_OUTPUT_DATA: *mut u32 = 0x400E1034 as *mut u32;

// Bit mask for PB27. This is pin 13 (the built-in LED) on the Arduino Due.
pub const PB27_MASK: u32 = 0x08000000;

// Addresses of several registers used to control the real-time timer.
pub const TIMER_MODE_REGISTER : *mut   u32 = 0x400E1A30 as *mut   u32;
pub const TIMER_VALUE_REGISTER: *const u32 = 0x400E1A38 as *const u32;


