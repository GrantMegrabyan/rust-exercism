use crate::types::{Frame, HorizDir, VertDir};

pub struct Ball {
    pub x: u32,
    pub y: u32,
    pub vert_dir: VertDir,
    pub horiz_dir: HorizDir,
    pub speed: u32,
}

impl Ball {
    pub fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x >= frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y >= frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }

    pub fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => {
                self.x = if self.x < self.speed {
                    0
                } else {
                    self.x - self.speed
                }
            }
            HorizDir::Right => self.x += self.speed,
        }
        match self.vert_dir {
            VertDir::Up => {
                self.y = if self.y < self.speed {
                    0
                } else {
                    self.y - self.speed
                }
            }
            VertDir::Down => self.y += self.speed,
        }
    }
}
