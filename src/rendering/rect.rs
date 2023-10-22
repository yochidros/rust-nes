pub struct Rect {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

impl Rect {
    pub fn new(x1: usize, x2: usize, y1: usize, y2: usize) -> Self {
        Rect { x1, x2, y1, y2 }
    }
}
