use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use rand::{thread_rng, Rng};

fn random(max: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(0..=max)
}
//struct Mobs {
//    x: i32,
//    y: i32,
//    mob_icon: char,
//}

struct Character {
    playing: bool,
    pos_x: i32,
    pos_y: i32,
    map_border_h: u32,
    map_border_w: u32,
    icons: char,
    point: i8,
    coin_y: i32,
    coin_x: i32,
}

impl Character {
    pub fn init_char(width: u32, height: u32) -> Character {
        Character {
            playing: true,
            pos_y: 3,
            pos_x: 4,
            map_border_h: height,
            map_border_w: width,
            icons: '🦀',
            point: 0,
            coin_y: 7,
            coin_x: 8,
        }
    }

    pub fn control(&mut self, engine: &ConsoleEngine) {
        if engine.is_key_pressed(KeyCode::Char('z')) || engine.is_key_pressed(KeyCode::Up) {
            self.pos_y = self.pos_y + -1;
        }

        if engine.is_key_pressed(KeyCode::Char('s')) || engine.is_key_pressed(KeyCode::Down) {
            self.pos_y = self.pos_y + 1;
        }

        if engine.is_key_pressed(KeyCode::Char('q')) || engine.is_key_pressed(KeyCode::Left) {
            self.pos_x = self.pos_x + -2;
        }

        if engine.is_key_pressed(KeyCode::Char('d')) || engine.is_key_pressed(KeyCode::Right) {
            self.pos_x = self.pos_x + 2;
        } else {
            if engine.is_key_pressed(KeyCode::Char(' ')) {
                self.playing = false;
                self.pos_x = 4;
                self.pos_y = 3;
            }
        }
    }

    pub fn coin(&mut self) {
        if self.pos_x == self.coin_x && self.pos_y == self.coin_y {
            self.coin_x = random(self.map_border_w.try_into().unwrap());
            self.coin_y = random(self.map_border_h.try_into().unwrap());
            self.point = self.point + 1;
            println!("");
            println!("point = {}", self.point);
            // println!("x={}, y={}", self.coin_x, self.coin_y);

            if self.coin_x % 2 != 0 {
                self.coin_x = self.coin_x + 1
            }

            if self.coin_y % 2 == 0 {
                self.coin_y = self.coin_y - 1
            }
            if self.coin_y <= 0 {
                self.coin_y = self.coin_y + 1
            }

            if self.coin_y >= 12 {
                self.coin_y = self.coin_y - 1
            }
            // println!("{},{}", self.coin_x, self.coin_y)
        }
    }

    //pub fn mob_spawn (&mut self, engine: &mut ConsoleEngine) {
    //    let mob1 = Mobs {x : 4, y : 3, mob_icon: '🐍'};
    //    engine.set_pxl(mob1.x, mob1.y, pixel::pxl_fg(mob1.mob_icon, Color::Cyan))
    //}
}

fn main() {
    let height: u32 = 10;
    let width: u32 = 40;
    let mut character = Character::init_char(width, height);
    // let mob:Mobs = Character::init_mob();
    let mut engine = console_engine::ConsoleEngine::init(width, height, 10).unwrap();

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); // reset the screen
        engine.fill_rect(0, 0, 40, 20, pixel::pxl('🟩'));
        character.control(&engine);
        character.coin();
        // character.mob_spawn(&mut engine);
        engine.set_pxl(
            character.pos_x,
            character.pos_y,
            pixel::pxl_fg(character.icons, Color::Cyan),
        );
        engine.set_pxl(
            character.coin_x,
            character.coin_y,
            pixel::pxl_fg('🍘', Color::Cyan),
        );

        if engine.is_key_pressed(KeyCode::Char('a')) {
            // if the user presses 'a' :
            break; // exits app
        }

        engine.draw(); // draw the screen
    }
}

