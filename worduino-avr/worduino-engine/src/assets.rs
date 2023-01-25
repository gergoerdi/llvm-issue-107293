pub use asset_format::*;

use avr_progmem::progmem;
pub use avr_progmem::wrapper::ProgMem;

progmem!{
    pub static progmem PLAYER_BULLET: [u8; 4] = [0x55, 0xaa, 0x55, 0xaa];
    pub static progmem MONSTER_BULLET: [u8; 4] = [0xaa, 0x55, 0xaa, 0x55];

    pub static progmem LEVEL: WallsData = WallsData {
        horizontal_walls: [0x00; 6],
        vertical_walls: [0x00; 6],
    };
}
