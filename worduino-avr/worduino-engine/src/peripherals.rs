pub trait Peripherals {
    fn get_stripe(&self, x: u8, stripe: u8) -> u8;
    fn set_stripe(&mut self, x: u8, stripe: u8, val: u8);
    fn get_button(&self) -> bool;

    fn set_pixel(&mut self, x: u8, y: u8, val: bool) {
        let stripe = y / 8;
        let offset = y - stripe * 8;
        let old = self.get_stripe(x, stripe);
        let mask = 1 << offset;
        let new = if val { old | mask } else { old & !mask };
        self.set_stripe(x, stripe, new)
    }
}
