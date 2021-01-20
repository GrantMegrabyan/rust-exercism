use crate::ball::Ball;
use crate::types::{Frame, HorizDir, VertDir};
use std::fmt::{Display, Formatter};

pub struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    pub fn new(frame: Frame) -> Game {
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
            speed: 1,
        };
        Game { frame, ball }
    }

    pub fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let horiz_border = |fmt: &mut Formatter| -> std::fmt::Result {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            write!(fmt, "+\n")
        };

        let row = |fmt: &mut Formatter, ball: &Ball, row_num: u32| -> std::fmt::Result {
            write!(fmt, "|")?;
            for col_num in 0..self.frame.width {
                if row_num == ball.y && col_num == ball.x {
                    write!(fmt, "o")?;
                } else {
                    write!(fmt, " ")?;
                }
            }
            write!(fmt, "|\n")
        };

        horiz_border(fmt)?;
        for row_num in 0..self.frame.height {
            row(fmt, &self.ball, row_num)?;
        }
        horiz_border(fmt)
    }
}
