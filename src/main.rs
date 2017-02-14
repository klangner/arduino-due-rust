#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate sam3x;

pub mod rust_base;
use sam3x::*;
use sam3x::pio::{PIO_B, P27};


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

/// Main function connected to the reset handler
/// Arduino Led is connected to the controller: B, pin: 27
fn start() -> ! {
    unsafe {
        // Enable PB27 (pin 13) and configure it for output.
        (*PIO_B).pio_enable    = P27;
        (*PIO_B).output_enable = P27;

        // Set the timer to a resolution of a millisecond.
        *TIMER_MODE_REGISTER = 0x00000020;

        // Continuously set and clear output on PB27 (pin 13). This
        // blinks the Due's built-in LED, which is the single
        // purpose of this program.
        loop {
            (*PIO_B).set_output_data = P27;
            sleep_ms(200);
            (*PIO_B).clear_output_data = P27;
            sleep_ms(800);
        }
    }
}
