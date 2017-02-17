#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate sam3x;

pub mod rust_base;
use sam3x::*;
use sam3x::drivers::led::{Led, Port};
use sam3x::hardware::rtt;


#[link_section=".vectors"]
pub static VECTOR_TABLE: VectorTable =
    VectorTable {
        reset_handler : start,
        exceptions    : [0; 14],
    };


/// Main function connected to the reset handler
/// Arduino Led is connected to the controller B, line 27
fn start() -> ! {
    let pb27 = Led::connect(Port::B, 27).expect("Wrong pin for led.");

    //We need to initialize timer before we can use it
    rtt::init_timer();

    // Blink led
    loop {
        pb27.on();
        rtt::wait(200);
        pb27.off();
        rtt::wait(900);
    }
}
