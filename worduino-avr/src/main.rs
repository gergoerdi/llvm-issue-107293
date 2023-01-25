#![feature(abi_avr_interrupt)]
#![feature(core_intrinsics)]
#![feature(asm_experimental_arch)]

#![no_std]
#![no_main]

#![allow(unused_variables)]
#![allow(dead_code)]

extern crate avr_std_stub;
extern crate avr_config;
extern crate ruduino;
extern crate avr_progmem;
extern crate worduino_engine as worduino;

use worduino::*;

use ruduino::Pin;
use ruduino::Register;
use ruduino::cores::current::*;

mod spi;
mod ssd1306;
mod timer;

struct ArduboyPeripherals {
    pub framebuffer: ssd1306::FrameBuffer,
}

impl ArduboyPeripherals {
    fn new() -> ArduboyPeripherals {
        ArduboyPeripherals {
            framebuffer: ssd1306::FrameBuffer::new(),
        }
    }
}

impl Peripherals for ArduboyPeripherals {
    fn get_button(&self) -> bool {
        port::B4::is_low()
    }

    fn get_stripe(&self, x: u8, stripe: u8) -> u8 {
        self.framebuffer.get_stripe(x, stripe)
    }

    fn set_stripe(&mut self, x: u8, stripe: u8, val: u8) {
        self.framebuffer.set_stripe(x, stripe, val);
    }
}

#[no_mangle]
pub extern fn main() {
    port::E6::set_input();
    port::E6::set_high();

    spi::setup();
    ssd1306::setup();
    timer::setup();

    let peripherals = ArduboyPeripherals::new();
    let mut engine = Engine::new(peripherals);

    loop {
        ssd1306::send(&engine.peripherals.framebuffer);
        engine.step();
        timer::wait_frame();
    }
}
