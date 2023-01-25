use ruduino::Pin;
use ruduino::delay::*;
use ruduino::cores::current::*;
use avr_progmem::progmem;

use spi;

progmem! {
    static progmem INIT_SCREEN: [u8; 13] =
        [ 0xD5, 0xf0,  // Set Display Clock Divisor 0xF0
          0x8D, 0x14,  // Charge Pump Setting 0x14
          0xA1,        // Set Segment Re-map (A0) | (b0001)
          0xC8,        // Set COM Output Scan Direction
          0x81, 0xCF,  // Set Contrast 0xCF
          0xD9, 0xF1,  // Set Precharge 0xF1
          0xAF,        // Display On
          0x20,        // Set display mode
          0x00,        // Horizontal addressing mode
        ];
}

type CS = port::D6;
type RST = port::D7;
type DC = port::D4;

pub fn setup() {
    CS::set_output();
    RST::set_output();
    DC::set_output();

    // Reset
    RST::set_high();
    delay(1);
    RST::set_low();
    delay_ms(10);
    RST::set_high();

    // Initialize
    CS::set_low();
    DC::set_low();
    CS::set_low();
    for i in 0..INIT_SCREEN.len() {
        spi::send(INIT_SCREEN.load_at(i))
    }
    DC::set_high();
    CS::set_high();
}

pub const SCREEN_WIDTH: u8 = 128;
pub const SCREEN_HEIGHT: u8 = 64;

pub struct FrameBuffer {
    data : [[u8; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize / 8],
}

impl FrameBuffer {
    pub fn new() -> FrameBuffer {
        FrameBuffer {
            data: [[0; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize / 8],
        }
    }

    pub fn get_stripe(&self, x: u8, stripe: u8) -> u8 {
        self.data[stripe as usize][x as usize]
    }

    pub fn set_stripe(&mut self, x: u8, stripe: u8, val: u8) {
        self.data[stripe as usize][x as usize] = val;
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, val: bool) {
        let stripe = y / 8;
        let offset = y - stripe * 8;
        let old = self.data[stripe as usize][x as usize];
        let mask = 1 << offset;
        let new = if val { old | mask } else { old & !mask };
        self.data[stripe as usize][x as usize] = new;
    }
}

pub fn send(fb: &FrameBuffer) {
    CS::set_low();
    for stripe in &fb.data {
        for bar in stripe {
            spi::send(*bar);
        }
    }
    CS::set_high();
}
