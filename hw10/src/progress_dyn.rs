pub trait Progress {
    fn build(&self, pos: f32) -> String;
    fn draw(&self, pos: f32);
}

pub struct BaseProgress {}

impl Progress for BaseProgress {
    fn build(&self, pos: f32) -> String {
        format!("\r{}%", pos)
    }

    fn draw(&self, pos: f32) {
        print!("{}", self.build(pos));
    }
}

pub struct BracketsDecorator {
    decorated: Box<dyn Progress>,
}

impl BracketsDecorator {
    pub fn new(decorated: Box<dyn Progress>) -> Self {
        Self { decorated }
    }
}

impl Progress for BracketsDecorator {
    fn build(&self, pos: f32) -> String {
        let mut s = self.decorated.build(pos);
        s.insert(1, '[');
        s.push(']');
        s
    }
    fn draw(&self, pos: f32) {
        print!("{}", self.build(pos));
    }
}

pub struct ProgressBarDecorator {
    decorated: Box<dyn Progress>,
}

impl ProgressBarDecorator {
    pub fn new(decorated: Box<dyn Progress>) -> Self {
        Self { decorated }
    }
}

impl Progress for ProgressBarDecorator {
    fn build(&self, pos: f32) -> String {
        let ret = termsize::get();
        if ret.is_none() {
            return "Can't get rows,cols of terminal".to_string();
        }

        let mut s = self.decorated.build(pos);
        let size = ret.unwrap();
        let mut cols = (size.cols as f32 / 100.0 * pos) as usize;
        if cols < s.len() {
            cols = 1;
        } else {
            cols -= s.len();
        }

        let line = format!("{:=>cols$}", ">");
        s.insert_str(1, &line);
        s
    }
    fn draw(&self, pos: f32) {
        print!("{}", self.build(pos));
    }
}
