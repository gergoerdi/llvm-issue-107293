use prelude::*;
use peripherals::*;
use assets::*;

pub struct Engine<P: Peripherals> {
    pub peripherals: P,
    state: State,
}

impl<P: Peripherals> Engine<P> {
    pub fn new(peripherals: P) -> Engine<P> {
        let mut player = Player::new();
        Engine{
            peripherals: peripherals,
            state: State::Playing{ level_state: LevelState::new(&mut player) },
        }
    }

    pub fn step(&mut self) {
        clear_screen(&mut self.peripherals);

        match &self.state {
            State::GameOver{ score } => {
            },

            State::Playing{ mut level_state } => {
                self.state = level_state.step(self);
            }
        }
    }

    fn read_button(&mut self) -> bool {
        self.peripherals.get_button()
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn fill_screen(p: &mut impl Peripherals, value: u8) {
    for x in 0..SCREEN_WIDTH {
        for stripe in 0..SCREEN_HEIGHT / 8 {
            p.set_stripe(x, stripe, value)
        }
    }
}

fn clear_screen(p: &mut impl Peripherals) {
    fill_screen(p, 0x00)
}

fn draw_sprite(p: &mut impl Peripherals, sprite: ProgMem<[u8]>, pos: (u8, u8)) {
    let (x0, y0) = pos;

    for i in 0..sprite.len() {
        let dx = i as u8;
        let mut col = sprite.load_at(i);
        for dy in 0..8 {
            p.set_pixel(x0 + dx, y0 + dy, col & 1 != 0);
            col >>= 1;
        }
    }
}

#[derive(Clone, Copy)]
struct Entity {
    pos: (u8, u8),
    d: Dir,
}

fn decode_pos(pos: (u8, u8)) -> ((u8, u8), (u8, u8)) {
    let (x, y) = pos;

    let xblock = (x - BLOCK_X_START) / BLOCK_WIDTH;
    let xoff   = (x - BLOCK_X_START) % BLOCK_WIDTH;
    let yblock = (y - BLOCK_Y_START) / BLOCK_HEIGHT;
    let yoff   = (y - BLOCK_Y_START) % BLOCK_HEIGHT;

    ((xblock, yblock), (xoff, yoff))
}

fn apply_move(pos: (u8, u8), d: (i8, i8)) -> (u8, u8) {
    let (x, y) = pos;
    let (dx, dy) = d;
    (x.saturating_add_signed(dx), y.saturating_add_signed(dy))
}

fn blocked_by_walls(level: &LevelState, e: Entity) -> Option<(u8, u8)> {
    let Entity{pos: pos@(_, y), .. } = e;
    let ((xblock, yblock), (xoff, yoff)) = decode_pos(pos);

    let (dx, dy) = (-1, 0);
    let blocked =
        if (xoff != 1 && dy != 0) || (yoff != 1 && dx != 0) {
            // Middle of a block
            true
        } else if dx != 0 && xoff == 1 {
            // Check vertical walls when moving horizontally
            let row = level.walls.vertical_walls[yblock as usize];
            if dx < 0 {
                if xblock == 0 {
                    if yblock == 2 && level.portals_open {
                        return Some((BLOCK_X_END - BLOCK_WIDTH + 1, y));
                    } else {
                        true
                    }
                } else {
                    // Wall to the left
                    row & (1 << (xblock - 1)) != 0
                }
            } else {
                // Wall to the right
                row & (1 << xblock) != 0
            }
        } else {
            false
        };

    if !blocked {
        Some(apply_move(e.pos, (dx, dy)))
    } else {
        None
    }
}

fn move_bullet(level: &LevelState, bullet: &mut Option<Entity>) {
    if let Some(e) = bullet {
        match blocked_by_walls(level, *e) {
            Some(pos) => { e.pos = pos },
            None => { *bullet = None },
        }
    }
}

enum State {
    Playing {
        level_state: LevelState,
    },
    GameOver {
        score: u16,
    },
}

#[derive(Clone, Copy)]
struct LevelState {
    walls: Walls,
    player: Player,
    monsters: [Option<BasicMonster>; 8],
    portals_open: bool,
}

impl LevelState {
    fn new(player: &mut Player) -> Self {
        let layout = LEVEL.load();

        LevelState {
            walls: layout.into(),
            portals_open: true,
            player: *player,
            monsters: [None; 8],
        }
    }

    fn draw<P: Peripherals>(&mut self, engine: &mut Engine<P>) {
        self.player.draw(&mut engine.peripherals);
        for monster in self.monsters.iter() {
            if let Some(monster) = monster {
                monster.draw(&mut engine.peripherals);
            }
        }
    }

    fn step<P: Peripherals>(&mut self, engine: &mut Engine<P>) -> State {
        self.draw(engine);

        let pressed = engine.read_button();

        let mut player = self.player;
        player.action(&self, pressed);

        let mut monsters = self.monsters;
        let mut monster_count: u8 = 0;
        for monster_slot in monsters.iter_mut() {
            if monster_slot.is_some() {
                monster_count += 1;
            }
        }

        self.player = player;
        self.monsters = monsters;

        if self.player.lives == u8::MAX {
            return State::GameOver{ score: 0 };
        }

        if monster_count == 0 {
            State::Playing{ level_state: Self::new(&mut self.player) }
        } else {
            State::Playing{ level_state: *self }
        }
    }
}

#[derive(Clone, Copy)]
struct BasicMonster {
    bullet: Option<Entity>,
}

impl BasicMonster {
    fn draw(&self, p: &mut impl Peripherals) {
        if let Some(ref e) = self.bullet {
            draw_sprite(p, MONSTER_BULLET, e.pos);
        }
    }
}

#[derive(Clone, Copy)]
struct Player {
    e: Entity,
    lives: u8,
    score: u16,
    bullet: Option<Entity>,
}

impl Player {
    fn start_pos() -> Entity {
        Entity {
            pos: (40, 40),
            d: Dir::Left,
        }
    }

    fn new() -> Player {
        Player {
            e: Self::start_pos(),
            lives: 2,
            score: 0,
            bullet: None,
        }
    }

    fn draw(&self, p: &mut impl Peripherals) {
        let Player{ bullet, .. } = self;
        if let Some(ref e) = bullet {
            draw_sprite(p, PLAYER_BULLET, e.pos);
        }
    }

    fn action(&mut self, level: &LevelState, pressed: bool) {
        move_bullet(level, &mut self.bullet);

        if pressed && self.bullet.is_none() {
            self.bullet = Some(self.e);
        }
    }
}
