pub enum VertDir {
    Up,
    Down,
}

pub enum HorizDir {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
}