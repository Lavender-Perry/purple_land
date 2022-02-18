use crate::wasm4;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Square {
    pub pos: Point,
    pub size: u32,
}

impl Square {
    /// Moves the object on the x-axis by amt (amt < 0 is left, amt > 0 is right).
    /// Returns true if the object was moved, false if it is collided with something.
    pub fn move_x(&mut self, amt: i32) -> bool {
        self.pos.x += amt;
        if self.pos.x < 0 {
            self.pos.x = 0;
            return false;
        }
        let upper_limit = 160 - self.size as i32;
        if self.pos.x > upper_limit {
            self.pos.x = upper_limit;
            return false;
        }
        return true;
    }

    /// Moves the object on the y-axis by amt (amt < 0 is up, amt > 0 is down)
    /// Returns true if the object was moved, false if it is collided with something.
    pub fn move_y(&mut self, amt: i32) -> bool {
        self.pos.y += amt;
        if self.pos.y < 0 {
            self.pos.y = 0;
            return false;
        }
        let upper_limit = 160 - self.size as i32;
        if self.pos.y > upper_limit {
            self.pos.y = upper_limit;
            return false;
        }
        return true;
    }

    /// Draws the square to the screen
    pub fn draw(&self) {
        wasm4::rect(self.pos.x, self.pos.y, self.size, self.size);
    }
} 
