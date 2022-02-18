use rand::prelude::*;
use rand_pcg::Pcg64;
use super::basics::{Square, Point};

const SIZE: u32 = 6;

pub struct Projectile {
    square: Square,
    direction: Point,
    bounced: bool,
}

impl Projectile {
    /// Generates a new projectile using rng.
    pub fn new(rng: &mut Pcg64) -> Self {
        let dir = if rng.gen() { 1 } else { -1 };
        Self {
            square: Square {
                pos: Point {
                    x: rng.gen_range(0..160 - (SIZE as i32)),
                    y: rng.gen_range(0..160 - (SIZE as i32)),
                },
                size: SIZE,
            },
            direction: if rng.gen() {
                Point {
                    x: dir,
                    y: 0,
                }
            } else {
                Point {
                    x: 0,
                    y: dir,
                }
            },
            bounced: false,
        }
    }

    /// Moves the projectile in its direction, bouncing if it collides.
    /// Returns if it has bounced twice.
    pub fn mv(&mut self) -> bool {
        let bounced = self.bounced;
        if bounced {
            return self.move_x() || self.move_y();
        }
        self.bounced = self.move_x() || self.move_y();
        return false;
    }
    
    /// Bounces projectile off player if they are overlapping,
    /// resets bounced flag if they bounce.  Draws projectile to the screen.
    pub fn update(&mut self, player: &Square) {
        if self.overlaps(player) {
            self.bounced = false;

            const MID: i32 = (SIZE / 2) as i32;
            let p_mid: i32 = (player.size / 2) as i32;
            // This method makes it possible for the player to freeze the projectile,
            // but I like it so it is intentional.
            self.direction = Point {
                x: -(player.pos.x + p_mid - self.square.pos.x - MID) / 10,
                y: -(player.pos.y + p_mid - self.square.pos.y - MID) / 10,
            };
        }
        
        self.square.draw();
    }
    
    /// Returns if self overlaps s (a Square)
    fn overlaps(&self, s: &Square) -> bool {
        let x_diff = self.square.pos.x - s.pos.x;
        let x_overlaps = if x_diff >= 0 {
            x_diff <= s.size as i32
        } else {
            x_diff > -(SIZE as i32)
        };

        let y_diff = self.square.pos.y - s.pos.y;
        let y_overlaps = if y_diff >= 0 {
            y_diff <= s.size as i32
        } else {
            y_diff > -(SIZE as i32)
        };

        x_overlaps && y_overlaps
    }
    
    fn move_x(&mut self) -> bool {
        if self.direction.x != 0 && !self.square.move_x(self.direction.x) {
            self.direction.x = -self.direction.x;
            return true;
        }
        return false;
    }
    
    fn move_y(&mut self) -> bool {
        if self.direction.y != 0 && !self.square.move_y(self.direction.y) {
            self.direction.y = -self.direction.y;
            return true;
        }
        return false;
    }
} 
