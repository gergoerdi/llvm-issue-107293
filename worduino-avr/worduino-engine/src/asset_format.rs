use prelude::*;

#[derive(Clone, Copy)]
pub struct Walls {
    pub vertical_walls: [u16; LEVEL_HEIGHT as usize],
    pub horizontal_walls: [u8; LEVEL_WIDTH as usize],
}

impl From<WallsData> for Walls {
    fn from(walls_data: WallsData) -> Self {
        Walls {
            vertical_walls: mirror_v(walls_data.vertical_walls),
            horizontal_walls: mirror_h(walls_data.horizontal_walls),
        }
    }
}

#[derive(Clone, Copy)]
pub struct WallsData {
    pub vertical_walls: [u8; LEVEL_HEIGHT as usize],
    pub horizontal_walls: [u8; ((LEVEL_WIDTH + 1) / 2) as usize],
}

const fn reverse_bits(mut x: u8) -> u8 {
    let mut y: u8 = 0;
    let mut i = 7;
    while i != 0 {
        if x & 0x01 != 0 {
            y |= 0x80;
        }
        y = y.rotate_left(1);
        x >>= 1;
        i -= 1;
    }
    y
}

const fn mirror_v<const N: usize> (walls: [u8; N]) -> [u16; N] {
    let mut r: [u16; N] = [0; N];
    let mut i = 0;
    while i != walls.len() {
        let x = walls[i];
        let y = reverse_bits(x) >> 2;
        r[i] = ((y as u16) << 5) | (x as u16);
        i += 1;
    }
    r
}

const fn mirror_h<const N: usize> (walls: [u8; N]) -> [u8; 2 * N - 1] {
    let mut r: [u8; 2 * N - 1] = [0; 2 * N - 1];
    let mut i = 0;
    while i != r.len() {
        let j = if i < N { i } else { 2 * N - (i + 2) };
        r[i] = walls[j];
        i += 1;
    }
    r
}
