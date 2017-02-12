#![feature(lang_items)]
#![no_std]
#![no_main]


pub mod rust_base;

// -------------------------------------------------------------------------------------------------
// Here starts the coded needed by controller
// -------------------------------------------------------------------------------------------------

// This is the top of the stack, as provided to us by the linker.
extern {
    static _estack: u32;
}


// Type declaration for the vector table.
pub struct VectorTable {
    pub initial_stack_pointer_value: &'static u32,
    pub reset_handler              : fn(),

    pub other_interrupt_vectors: [u32; 44],
}

unsafe impl Sync for VectorTable {}


// The actual vector table.
#[link_section=".vectors"]
pub static VECTOR_TABLE: VectorTable = unsafe {
    VectorTable {
        initial_stack_pointer_value: &_estack,
        reset_handler              : start,
        other_interrupt_vectors    : [0; 44],
    }
};


// Addresses of several registers used to control parallel I/O.
const PB_PIO_ENABLE       : *mut u32 = 0x400E1000 as *mut u32;
const PB_OUTPUT_ENABLE    : *mut u32 = 0x400E1010 as *mut u32;
const PB_SET_OUTPUT_DATA  : *mut u32 = 0x400E1030 as *mut u32;
const PB_CLEAR_OUTPUT_DATA: *mut u32 = 0x400E1034 as *mut u32;

// Bit mask for PB27. This is pin 13 (the built-in LED) on the Arduino Due.
const PB27_MASK: u32 = 0x08000000;

// Addresses of several registers used to control the real-time timer.
const TIMER_MODE_REGISTER : *mut   u32 = 0x400E1A30 as *mut   u32;
const TIMER_VALUE_REGISTER: *const u32 = 0x400E1A38 as *const u32;


// -------------------------------------------------------------------------------------------------
//  And here is our program
// -------------------------------------------------------------------------------------------------

// As the name suggests, this function sleeps for a given number of
// milliseconds. Our replacement for Arduino's delay function.
fn sleep_ms(milliseconds: u32) {
    unsafe {
        let sleep_until = *TIMER_VALUE_REGISTER + milliseconds;
        while *TIMER_VALUE_REGISTER < sleep_until {}
    }
}

// This function is the entry point for our application and the handler
// function for the reset interrupt.
fn start() {
    unsafe {
        // Enable PB27 (pin 13) and configure it for output.
        *PB_PIO_ENABLE    = PB27_MASK;
        *PB_OUTPUT_ENABLE = PB27_MASK;

        // Set the timer to a resolution of a millisecond.
        *TIMER_MODE_REGISTER = 0x00000020;

        // Continuously set and clear output on PB27 (pin 13). This
        // blinks the Due's built-in LED, which is the single
        // purpose of this program.
        loop {
            *PB_SET_OUTPUT_DATA = PB27_MASK;
            sleep_ms(1000);
            *PB_CLEAR_OUTPUT_DATA = PB27_MASK;
            sleep_ms(1000);
        }
    }
}
