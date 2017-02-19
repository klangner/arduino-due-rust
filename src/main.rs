#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate sam3x;

pub mod rust_base;
use sam3x::*;
use sam3x::hardware::peripherals::{Peripheral};
use sam3x::drivers::led::{Led};
use sam3x::drivers::button::{Button};
use sam3x::hardware::rtt::{init_timer, wait_ms};
use sam3x::hardware::pmc::{enable_peripheral_clk};
use sam3x::hardware::wdt;


#[link_section=".vectors"]
pub static VECTOR_TABLE: VectorTable =
    VectorTable {
        reset_handler : start,
        exceptions    : [0; 14],
    };


/// Main function connected to the reset handler
/// Arduino Led is connected to the controller B, line 27
fn start() -> ! {

    // We need to initialize board before we can use it
    // We need timer
    init_timer();
    // We need to enable clock for Port C before we can use it as input
    enable_peripheral_clk(Peripheral::PioC);

    // Now we can initialize state
    let led = Led::connect(Peripheral::PioB, 27).expect("Wrong pin for led.");
    let button = Button::connect(Peripheral::PioC, 22).expect("Wrong pin for button.");

    // We execute functionality in never ending loop
    loop {
        wdt::restart_watchdog();
        wait_ms(100);
        if button.is_pressed() {
            led.on()
        };
    }
}
