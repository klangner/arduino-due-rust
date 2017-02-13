#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate sam3x;

pub mod rust_base;
use sam3x::*;


#[link_section=".vectors"]
pub static VECTOR_TABLE: VectorTable =
    VectorTable {
        reset_handler              : start,
        other_interrupt_vectors    : [0; 44],
    };


// As the name suggests, this function sleeps for a given number of milliseconds.
fn sleep_ms(milliseconds: u32) {
    unsafe {
        let sleep_until = *TIMER_VALUE_REGISTER + milliseconds;
        while *TIMER_VALUE_REGISTER < sleep_until {}
    }
}

// this function is called
// function for the reset interrupt.
fn start() -> ! {
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
            sleep_ms(500);
            *PB_CLEAR_OUTPUT_DATA = PB27_MASK;
            sleep_ms(500);
        }
    }
}
