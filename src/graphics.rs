use std::path::PathBuf;


#[derive(Clone)]
pub struct Line {
    pub start: (i32, i32),
    pub end: (i32, i32)
}

impl Line {
    pub fn create(start: (i32, i32), end: (i32, i32)) -> Line {
        Line {
            start: start,
            end: end
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.start = (self.start.0 + x, self.start.1 + y);
        self.end = (self.end.0 + x, self.end.1 + y);
    }
}

#[derive(Clone)]
pub enum Graphics {
    Line(Vec<Line>),
    Image {
        path: PathBuf
    }
}