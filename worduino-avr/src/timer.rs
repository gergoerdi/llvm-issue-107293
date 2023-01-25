use core::arch::asm;
use avr_config::CPU_FREQUENCY_HZ;

use ruduino::interrupt::*;
use ruduino::Register;
use ruduino::cores::current::*;

pub fn setup() {
    without_interrupts(|| {
        // 16 MHz / 1024 / 256 = 61 Hz, so this gives us almost 60 fps.
        TCCR0B::write(TCCR0B::CS00 | TCCR0B::CS02); // Divide by 1024
        TCCR0A::write(0); // count UP to 0xff
    })
}

pub fn wait_frame() {
    TIFR0::wait_until_set(TIFR0::TOV0);
    TIFR0::set(TIFR0::TOV0);
}

pub fn sleep_ms(duration_ms: u16) {
    const CYCLES_PER_MS: u16 = (CPU_FREQUENCY_HZ / 1000) as u16;
    const CYCLES_PER_INNER_LOOP: u16 = 6;
    const INNER_LOOP_ITERATIONS: u16 = CYCLES_PER_MS / CYCLES_PER_INNER_LOOP;

    let mut outer = 0;
    while outer < duration_ms {
        let mut inner = 0;
        while inner < INNER_LOOP_ITERATIONS {
            unsafe { asm!(""); }
            inner += 1;
        }
        outer += 1;
    }
}
