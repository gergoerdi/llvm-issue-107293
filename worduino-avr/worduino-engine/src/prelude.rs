use avr_progmem::wrapper::ProgMem;

pub const SCREEN_WIDTH: u8 = 128;
pub const SCREEN_HEIGHT: u8 = 64;

pub const BLOCK_X_START: u8 = 9;
pub const BLOCK_Y_START: u8 = 2;
pub const BLOCK_WIDTH: u8 = 10;
pub const BLOCK_HEIGHT: u8 = 10;
pub const LEVEL_WIDTH: u8 = 11;
pub const LEVEL_HEIGHT: u8 = 6;
pub const BLOCK_X_END: u8 = BLOCK_X_START + BLOCK_WIDTH * LEVEL_WIDTH;
pub const BLOCK_Y_END: u8 = BLOCK_Y_START + BLOCK_HEIGHT * LEVEL_HEIGHT;
