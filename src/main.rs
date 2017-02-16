#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate sam3x;

pub mod rust_base;
use sam3x::*;
use sam3x::pio;
use sam3x::rtt;


#[link_section=".vectors"]
pub static VECTOR_TABLE: VectorTable =
    VectorTable {
        reset_handler              : start,
        other_interrupt_vectors    : [0; 44],
    };


/// Main function connected to the reset handler
/// Arduino Led is connected to the controller: B, pin: 27
fn start() -> ! {
    let pb27 = pio::pin(pio::Port::B, 27, pio::Mode::Output).expect("Can't connect to the led");

    //We need to initialize timer before we can use it
    rtt::init_timer();

    // Blink led
    loop {
        pb27.on();
        rtt::wait(100);
        pb27.off();
        rtt::wait(100);
    }
}
