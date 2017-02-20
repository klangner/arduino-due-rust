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
use sam3x::hardware::pmc;
use sam3x::hardware::wdt;
use sam3x::hardware::uart;


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
    pmc::enable_peripheral_clock_0(Peripheral::PioC);

    // Now we can initialize state
    let led = Led::connect(Peripheral::PioB, 27).expect("Wrong pin for led.");
    let button = Button::connect(Peripheral::PioC, 22).expect("Wrong pin for button.");
    let tx = uart::Tx::init(uart::BR_9600);

    // We execute functionality in never ending loop
//    tx.write("Initialize\n");
    let mut last_pressed = false;
    let mut led_on = true;
    led.on();
    loop {
        tx.write("a");
        wdt::restart_watchdog();
        wait_ms(200);
        if button.is_pressed(){
            if last_pressed == false {
                tx.write("Button pressed\n");
                if led_on {
                    led.off()
                } else {
                    led.on()
                }
                led_on = !led_on;
            }
            last_pressed = true;
        } else {
            last_pressed = false;
        }
    }
}
